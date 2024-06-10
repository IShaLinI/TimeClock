create table timecards (
    user_id int not null,
    start_time bigint not null,
    end_time bigint not null,
    description text not null
);

create unique index timecard_user_id_start_time_end_time_description on timecards (user_id, start_time, end_time, description);