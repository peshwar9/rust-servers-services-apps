
drop table if exists ezyweb_user;

/* Create tables. */
/* Note: Don't put a comma after last field */

create table ezyweb_user
(
    username varchar(20) primary key,
    tutor_id INT,    
    user_password CHAR(100) not null
);

grant all privileges on table ezyweb_user to truuser;
grant all privileges on all sequences in schema public to truuser;
