use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestMessageArgs,
        CreateChatCompletionRequestArgs, Role,
    },
    Client as OpenAIClient,
};
use errors::Error;
use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use std::sync::{Arc, Mutex};
use tokio_postgres::{Client as DbClient, NoTls};
use async_recursion::async_recursion;

use std::fs;

pub mod config;
pub mod connection;
pub mod errors;
pub mod openai;

#[derive(Debug, Clone)]
pub struct Session {
    pub connection_id: u8,
    pub db_client: Arc<Mutex<Option<DbClient>>>,
    pub open_ai_client: OpenAIClient<OpenAIConfig>,
    pub messages: Vec<ChatCompletionRequestMessage>,
    // pub functions: [ChatCompletionFunctions; 1],
}

pub async fn init_session(id: u8, config: config::Config) -> Result<Session, Error> {
    //init db connection
    let connection = config.get_connection(id).clone();
    let connection_id = connection.id;
    let url = connection.url().clone();

    let db_client = match connection.sslmode {
        Some(sslmode) => match sslmode.as_str() {
            "require" => connect_ssl(url, id).await?,
            _ => connect(url).await?,
        },
        None => connect(url).await?,
    };

    // get database schema

    // init open ai client

    let api_key = config.openai.token.as_str();
    let config = OpenAIConfig::new().with_api_key(api_key);
    let open_ai_client = OpenAIClient::with_config(config);

    let system_msg = r#"
        You are a database analyst. You can run SQL queries and get results using function run_sql_query.
        You get requests from user and you need to run queries and get results.
        
        Here is database schema:

        CREATE TABLE public.users (
            id bytea NOT NULL,
            elo float8 NOT NULL DEFAULT 0,
            "eloBox" int4 NOT NULL DEFAULT 5,
            "extraQuestions" _varchar NULL,
            "lastActive" timestamp(0) NULL,
            "location" public.geometry NULL,
            prime timestamp(0) NULL,
            "pushToken" varchar(255) NULL,
            "phoneNumber" varchar(255) NULL,
            settings jsonb NULL,
            "signUpInProgress" bool NOT NULL DEFAULT true,
            location_time timestamp(0) NULL,
            "badgeCount" int4 NOT NULL DEFAULT 0,
            stats jsonb NULL,
            inserted_at timestamp(0) NOT NULL,
            updated_at timestamp(0) NOT NULL,
            point public.geometry(point, 4326) NULL,
            boost bool NOT NULL DEFAULT false,
            CONSTRAINT users_pkey PRIMARY KEY (id)
        );
        CREATE INDEX user_point_index ON public.users USING gist (point);

        CREATE TABLE public.user_settings (
            id bigserial NOT NULL,
            birth timestamp(0) NULL,
            birthplace varchar(255) NULL,
            "crossPath" bool NOT NULL DEFAULT true,
            description text NULL,
            "filter" jsonb NULL,
            "genderDescription" varchar(255) NULL,
            hidden bool NOT NULL DEFAULT false,
            "isFemale" bool NOT NULL DEFAULT false,
            "isMale" bool NOT NULL DEFAULT false,
            locale varchar(255) NULL,
            media _varchar NULL,
            "updatedMediaAt" timestamp(0) NULL,
            "name" varchar(255) NULL,
            notifications jsonb NULL,
            residence varchar(255) NULL,
            school varchar(255) NULL,
            "work" varchar(255) NULL,
            email varchar(255) NULL,
            user_id bytea NULL,
            CONSTRAINT user_settings_pkey PRIMARY KEY (id),
            CONSTRAINT user_settings_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE
        );
        CREATE UNIQUE INDEX user_settings_user_id_index ON public.user_settings USING btree (user_id);
        "#;

    let messages = vec![ChatCompletionRequestMessageArgs::default()
        .role(Role::System)
        .content(system_msg)
        .build()
        .unwrap()];

    // init session

    Ok(Session {
        connection_id,
        db_client,
        open_ai_client,
        messages,
    })
}

async fn connect(url: String) -> Result<Arc<Mutex<Option<DbClient>>>, Error> {
    println!("connect: {:?}", url);
    let (client, connection) = tokio_postgres::connect(url.as_str(), NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(Arc::new(Mutex::new(Some(client))))
}

async fn connect_ssl(url: String, id: u8) -> Result<Arc<Mutex<Option<DbClient>>>, Error> {
    println!("connect_ssl: {:?}", url);
    let cert = fs::read("ca-certificate.crt")?;
    let cert = Certificate::from_pem(&cert)?;
    let connector = TlsConnector::builder().add_root_certificate(cert).build()?;
    let connector = MakeTlsConnector::new(connector);
    let (client, connection) = tokio_postgres::connect(url.as_str(), connector).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(Arc::new(Mutex::new(Some(client))))
}

#[async_recursion]
pub async fn exec(input: String, session: Session) -> Result<Session, Error> {
    // process input using openai
    let new_msg = ChatCompletionRequestMessageArgs::default()
        .role(Role::User)
        .content(input)
        .build()?;

    let mut messages = session.messages.clone();

    messages.push(new_msg);

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo-0613")
        .messages(messages.clone())
        .functions(openai::functions::list_functions())
        .function_call("auto")
        .build()?;

    let response_message = session
        .open_ai_client
        .chat()
        .create(request)
        .await?
        .choices
        .get(0)
        .unwrap()
        .message
        .clone();

    if let Some(function_call) = response_message.function_call {
        let function_args: serde_json::Value = function_call.arguments.parse().unwrap();
        let query = function_args["query"].as_str().unwrap();

        println!("query: {:?}", query);

        // db client lock and query

        let client = session.db_client.lock().unwrap().take().unwrap();

        match client.query(query, &[]).await {
            Ok(rows) => {
                println!("rows: {:?}", rows);
                let value: &str = rows[0].get(0);

                let fn_message = ChatCompletionRequestMessageArgs::default()
                    .role(Role::Assistant)
                    .content(value)
                    .build()?;

                messages.push(fn_message);

                let request = CreateChatCompletionRequestArgs::default()
                    .max_tokens(512u16)
                    .model("gpt-3.5-turbo-0613")
                    .messages(messages.clone())
                    .build()?;

                let response = session.open_ai_client.chat().create(request).await?;
                let response_message = response.choices.get(0).unwrap().message.clone();

                let assistant_message = ChatCompletionRequestMessageArgs::default()
                    .role(Role::Assistant)
                    .content(response_message.content.clone().unwrap_or_default())
                    .build()?;

                messages.push(assistant_message);

                println!("\nResponse:\n");
                for choice in response.choices {
                    println!(
                        "{}: Role: {}  Content: {:?}",
                        choice.index, choice.message.role, choice.message.content
                    );
                }
            }
            Err(e) => {
                println!("error: {:?}", e);
                let error_msg = "Error: ".to_string() + e.to_string().as_str();
                exec(error_msg, session.clone()).await?;
            }
        }

        // let rows = client.query(query, &[]).await?;

        *session.db_client.lock().unwrap() = Some(client);
    } else {
        let assistant_message = ChatCompletionRequestMessageArgs::default()
            .role(Role::Assistant)
            .content(response_message.content.clone().unwrap_or_default())
            .build()?;

        messages.push(assistant_message);
    }

    let session = Session {
        messages,
        ..session
    };

    Ok(session)
}
