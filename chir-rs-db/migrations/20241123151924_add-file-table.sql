-- Add migration script here

create table 'file' (
    id integer primary key not null,
    file_path text not null,
    mime text not null,
    b3hash blob not null
);
create unique index file_path_mime on 'file' (file_path, mime);
create index file_path on 'file' (file_path);