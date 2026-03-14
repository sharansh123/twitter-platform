create table public.user_follow
(
    follower_id  varchar(50) not null
        references public.users,
    followed_id varchar(50) not null
        references public.users,
    followed_at  timestamp default CURRENT_TIMESTAMP,
    PRIMARY KEY (followed_id, follower_id)
);

create sequence user_follow_offset_seq;

alter table public.user_follow add column offset_val integer default nextval('user_follow_offset_seq');

create index user_follow_offset_idx on public.user_follow (follower_id, offset_val);

create table public.users
(
    id            varchar(50)  not null
        primary key,
    name          varchar(50)  not null,
    password_hash varchar(255) not null,
    followers     integer   default 0,
    following     integer   default 0,
    created_at    timestamp default CURRENT_TIMESTAMP,
    updated_at    timestamp default CURRENT_TIMESTAMP
);

ALTER TABLE public.users
    ALTER COLUMN followers SET NOT NULL;

ALTER TABLE public.users
    ALTER COLUMN following SET NOT NULL;

alter table public.users
    owner to postgres;

create table public.posts
(
    id         integer generated always as identity
        primary key,
    user_id    varchar(50)  not null
        references public.users,
    content    varchar(255) not null,
    created_at timestamp default CURRENT_TIMESTAMP
);

alter table public.posts
    owner to postgres;

create function update_user_following() returns trigger as
    $$
    begin
       IF (TG_OP = 'INSERT') THEN
           UPDATE public.users SET followers = followers + 1 WHERE id = NEW.followed_id;
           UPDATE public.users SET following = following + 1 WHERE id = NEW.follower_id;
       end if;
       IF (TG_OP = 'DELETE') THEN
           UPDATE public.users SET followers = followers - 1 WHERE id = OLD.followed_id;
           UPDATE public.users SET following = following - 1 WHERE id = OLD.follower_id;
       end if;
       RETURN NEW;
    end;
    $$ LANGUAGE plpgsql;

create trigger update_user_trigger
    after insert or delete on public.user_follow
    for each row execute function update_user_following();