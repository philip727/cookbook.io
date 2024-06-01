create table users
(
    uid      serial,
    username varchar(16)  not null,
    email    varchar(100) not null,
    password varchar(255) not null,
    primary key (uid),
    unique (username),
    unique (email)
);

create table recipes
(
    id               serial,
    recipe_file_path varchar(255)                                       not null,
    user_id          integer                                            not null,
    date_created     timestamp with time zone default CURRENT_TIMESTAMP not null,
    primary key (id),
    constraint fk_user
        foreign key (user_id) references users
            on delete cascade
);
