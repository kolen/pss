create table users (
       id integer not null primary key autoincrement,
       name text not null unique,
       password text,
       created_at integer not null,
       updated_at integer not null
);

create table categories (
       id integer not null primary key autoincrement,
       user_id integer not null,
       name text,
       created_at integer not null,
       updated_at integer not null,

       foreign key(user_id) references users(id)
);

create index idx_categories_on_user_id_created_at on categories (user_id, created_at);

create table words (
       id integer not null primary key autoincrement,
       category_id integer not null,
       word text not null,
       created_at integer not null,
       updated_at integer not null,

       foreign key(category_id) references categories(id)
);

create index idx_words_on_category_id_created_at on words (category_id, created_at);

create table games (
       id integer not null primary key autoincrement,
       user_words_id integer not null,
       user_composed_id integer not null,
       created_at integer not null,

       foreign key(user_words_id) references users(id),
       foreign key(user_composed_id) references users(id)
);

create index idx_games_on_created_at on games (created_at);

create table sessions (
       id integer not null primary key autoincrement,
       user_id integer not null,
       secret text not null,
       created_at integer not null,
       last_used_at integer not null,
       created_user_agent text not null,

       foreign key(user_id) references users(id)
);

create index idx_sessions_on_secret on sessions (secret);
