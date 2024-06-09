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

create table user_details
(
    bio          varchar(255),
    display_name varchar(30),
    pronouns     varchar(20),
    location     varchar(50),
    user_id      integer,
    constraint user_details_pk
        unique (user_id),
    constraint user_details_user_details__fk
        foreign key (user_id) references users
            on delete cascade
);
