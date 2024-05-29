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
    id           serial,
    title        varchar(30)                                        not null,
    description  varchar(100)                                       not null,
    user_id      integer                                            not null,
    date_created timestamp with time zone default CURRENT_TIMESTAMP not null,
    primary key (id),
    constraint fk_user
        foreign key (user_id) references users
            on delete cascade
);

create table recipe_steps
(
    id          serial,
    recipe_id   integer      not null,
    description varchar(255) not null,
    step_order  integer      not null,
    primary key (id),
    constraint unique_recipe_step_order
        unique (recipe_id, step_order),
    constraint fk_recipe
        foreign key (recipe_id) references recipes
            on delete cascade
);
