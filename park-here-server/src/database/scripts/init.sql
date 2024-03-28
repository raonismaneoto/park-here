create table region(
    id varchar(255),
    latitude float,
    longitude float,
    primary key (id)
);

create table parking_vacancy(
    id varchar(255), 
    v_status integer, 
    t integer, 
    region_id varchar(255) references region,
    primary key (id)
);


insert into 
    region (id, latitude, longitude)
values 
    ('1', -11.299964, -41.856794),
    ('2', -11.300806, -41.855764),
    ('3', -11.300385, -41.854992),
    ('4', -11.304257, -41.855421);

insert into 
    parking_vacancy (id, v_status, t, region_id)
values
    ('1', 1, 1, '1'),
    ('2', 1, 1, '2'),
    ('3', 1, 1, '3'),
    ('4', 1, 1, '4');