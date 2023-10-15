# Trekker

Manage database migrations and seeds. Trekker is an incredibly specific tool used to migrate and seed databases. It does this in a very simplistic manner and I'd be surprised if I ever make it do more. I use it in my workflows and it's useful for me.

## Usage

### Connecting

Trekker uses the `DATABASE_URL` variable to connect to your Postgres database.

### Migrations

Trekker expects that you'll keep migrations in a particular directory.

Writing a migration entails creating a file, `db/migrations/001-add-reddit-subreddits.sql`, with the migration SQL:

```sql
create table if not exists subreddits (
  id serial primary key,
  subreddit varchar not null unique
);
```

I generally use a three digit padded prefix to control execution order but you could also use a timestamp or date.

To run all of the migrations, use the migrate command:

```shell
trekker migrate db/migrations
```

### Seeds

Trekker expects that you'll keep seeds in a particular directory.

Writing a seed entails creating a file, `db/seeds/001-add-some-subreddits.sql`, with the seed SQL:

```sql
insert into subreddits (subreddit) values
  ('mechmarket'),
  ('hardwareswap')
;
```



To run all of the seeds, use the seed command:

```shell
trekker seed db/seeds
```
