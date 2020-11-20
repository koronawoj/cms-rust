create table if no exists customers_list (
    guid serial primary key,
    first_name varchar(150),
    last_name varchar(150),
    email varchar(150),
    address varchar(150)
);

insert into customers_list (first_name, last_name, email, address) 
    values ('Dorothy1','Blum','dblumr8@furl.net','7334 Oak Valley Parkway'),
        ('Dorothy2','Blum','dblumr8@furl.net','7334 Oak Valley Parkway'),
        ('Dorothy3','Blum','dblumr8@furl.net','7334 Oak Valley Parkway');