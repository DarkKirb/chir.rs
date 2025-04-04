-- Add migration script here

create table file_map (
    id bigint generated always as identity,
    file_path text not null,
    mime text not null,
    b3hash bytea not null
);
create unique index file_path_mime on file_map (file_path, mime);
create index file_path on file_map (file_path);