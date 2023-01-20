create table users (
       id integer primary key autoincrement,
       name text not null,
       created_at integer not null,
       updated_at integer not null
);

create table categories (
       id integer primary key autoincrement,
       user_id integer not null,
       name text,
       created_at integer not null,
       updated_at integer not null,

       foreign key(user_id) references users(id)
);

create table words (
       id integer primary key autoincrement,
       category_id integer not null,
       word text not null,
       created_at integer not null,
       updated_at integer not null,

       foreign key(category_id) references categories(id)
);

create table games (
       id integer primary key autoincrement,
       user_words_id integer not null,
       user_composed_id integer not null,
       created_at integer not null,

       foreign key(user_words_id) references users(id),
       foreign key(user_composed_id) references users(id)
);
