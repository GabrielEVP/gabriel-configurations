---
name: database-mysql
description: "Trigger: mysql, mariadb, sql, database, query, indexing, mysqldump, schema, migration, innodb. MySQL/MariaDB administration — queries, indexing, EXPLAIN, backup, restore, optimization, migrations, user management."
license: Apache-2.0
metadata:
  author: gabriel
  version: "1.0"
---

# MySQL / MariaDB Administration

## Connection & Basics

```bash
# Connect to MySQL
mysql -u root -p
mysql -u root -p -h <host> -P <port>

# Connect inside Docker
docker compose exec db mysql -u root -p

# Execute a single query
mysql -u root -p -e "SHOW DATABASES;"

# Import a SQL file
mysql -u root -p <database> < dump.sql

# Export a database
mysqldump -u root -p <database> > dump.sql

# Export with no data (schema only)
mysqldump -u root -p --no-data <database> > schema.sql

# Export specific tables
mysqldump -u root -p <database> <table1> <table2> > tables.sql

# Export with compression
mysqldump -u root -p <database> | gzip > dump.sql.gz
```

## Database & User Management

```sql
-- Show databases
SHOW DATABASES;

-- Create database
CREATE DATABASE <dbname> CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- Use database
USE <dbname>;

-- Show tables
SHOW TABLES;

-- Create user
CREATE USER '<user>'@'localhost' IDENTIFIED BY '<password>';
CREATE USER '<user>'@'%' IDENTIFIED BY '<password>';

-- Grant privileges
GRANT ALL PRIVILEGES ON <db>.* TO '<user>'@'localhost';
GRANT SELECT, INSERT, UPDATE, DELETE ON <db>.* TO '<user>'@'%';

-- Grant with grant option
GRANT ALL PRIVILEGES ON <db>.* TO '<user>'@'localhost' WITH GRANT OPTION;

-- Show grants for user
SHOW GRANTS FOR '<user>'@'localhost';

-- Revoke privileges
REVOKE ALL PRIVILEGES ON <db>.* FROM '<user>'@'localhost';

-- Drop user
DROP USER '<user>'@'localhost';

-- Apply privilege changes
FLUSH PRIVILEGES;

-- Change password
ALTER USER '<user>'@'localhost' IDENTIFIED BY '<newpassword>';
```

## Schema & Table Operations

```sql
-- Show create table DDL
SHOW CREATE TABLE <table>;

-- Describe table structure
DESCRIBE <table>;
DESC <table>;

-- Show indexes
SHOW INDEXES FROM <table>;

-- Add column
ALTER TABLE <table> ADD COLUMN <col> <type> AFTER <other_col>;

-- Modify column
ALTER TABLE <table> MODIFY COLUMN <col> <new_type>;

-- Rename column
ALTER TABLE <table> RENAME COLUMN <old> TO <new>;

-- Drop column
ALTER TABLE <table> DROP COLUMN <col>;

-- Add index
CREATE INDEX idx_<col> ON <table>(<col>);
CREATE UNIQUE INDEX idx_unique_<col> ON <table>(<col>);

-- Composite index (column order matters!)
CREATE INDEX idx_<col1>_<col2> ON <table>(<col1>, <col2>);

-- Drop index
DROP INDEX idx_<col> ON <table>;

-- Add foreign key
ALTER TABLE <table> ADD CONSTRAINT fk_<name>
  FOREIGN KEY (<col>) REFERENCES <parent>(<id>)
  ON DELETE CASCADE;

-- Add composite primary key
ALTER TABLE <table> ADD PRIMARY KEY (<col1>, <col2>);
```

## Indexing Strategy

```sql
-- Use EXPLAIN to check query plan
EXPLAIN SELECT * FROM orders WHERE customer_id = 1;

-- EXPLAIN with extended info
EXPLAIN ANALYZE SELECT * FROM orders WHERE status = 'PENDING';

-- Show slow queries
-- Requires: set global slow_query_log = ON;
SHOW VARIABLES LIKE 'slow_query%';
SHOW VARIABLES LIKE 'long_query_time';
```

### Index Guidelines

```
- Index columns used in WHERE, JOIN, ORDER BY
- Composite index order: most selective column first
- Don't over-index — indexes slow down writes
- Use covering indexes when possible (include all selected columns)
- For LIKE queries, only prefix matches use indexes: 'abc%' works, '%abc' doesn't
- Keep indexes narrow — don't index large TEXT/BLOB columns
```

## Query Patterns

```sql
-- Pagination (offset-based)
SELECT * FROM orders
ORDER BY created_at DESC
LIMIT 20 OFFSET 0;

-- Pagination (keyset/cursor — more efficient for large offsets)
SELECT * FROM orders
WHERE created_at < :last_seen
ORDER BY created_at DESC
LIMIT 20;

-- Count with condition
SELECT COUNT(*) FROM orders WHERE status = 'PENDING';

-- Group by with count
SELECT status, COUNT(*) as cnt
FROM orders
GROUP BY status
ORDER BY cnt DESC;

-- Join with aggregation
SELECT c.name, COUNT(o.id) as order_count
FROM customers c
LEFT JOIN orders o ON o.customer_id = c.id
GROUP BY c.id, c.name
ORDER BY order_count DESC;

-- Find duplicates
SELECT email, COUNT(*)
FROM users
GROUP BY email
HAVING COUNT(*) > 1;

-- Update with join
UPDATE orders o
JOIN customers c ON c.id = o.customer_id
SET o.customer_name = c.name
WHERE c.is_active = 1;
```

## Performance Tuning

```sql
-- Show running queries
SHOW FULL PROCESSLIST;

-- Kill a query
KILL <thread_id>;

-- Show table sizes
SELECT
  table_name,
  ROUND(((data_length + index_length) / 1024 / 1024), 2) AS size_mb
FROM information_schema.TABLES
WHERE table_schema = '<database>'
ORDER BY size_mb DESC;

-- Show indexes with cardinality
SELECT
  index_name,
  seq_in_index,
  column_name,
  cardinality
FROM information_schema.STATISTICS
WHERE table_schema = '<database>' AND table_name = '<table>';

-- Show table status (row count, size, engine)
SHOW TABLE STATUS FROM <database>;

-- Check for table fragmentation
SELECT
  table_name,
  data_free / 1024 / 1024 as frag_mb
FROM information_schema.TABLES
WHERE data_free > 0 AND table_schema = '<database>';
```

## Transactions & Locking

```sql
-- Start transaction
START TRANSACTION;
-- or
BEGIN;

-- Commit
COMMIT;

-- Rollback
ROLLBACK;

-- Set isolation level
SET TRANSACTION ISOLATION LEVEL READ COMMITTED;
SET SESSION TRANSACTION ISOLATION LEVEL READ COMMITTED;

-- Show current locks
SHOW OPEN TABLES WHERE in_use > 0;

-- Check for blocking queries
SELECT * FROM information_schema.INNODB_LOCKS;

-- Show lock waits
SELECT * FROM information_schema.INNODB_LOCK_WAITS;
```

## Backup & Restore

```bash
# Full backup
mysqldump -u root -p --all-databases --routines --events > full_backup.sql

# Single database with routines
mysqldump -u root -p --routines --triggers --events <database> > db.sql

# Single table
mysqldump -u root -p <database> <table> > table.sql

# Backup without data (schema only)
mysqldump -u root -p --no-data <database> > schema.sql

# Compressed backup
mysqldump -u root -p <database> | gzip > db_$(date +%Y%m%d).sql.gz

# Restore
mysql -u root -p <database> < backup.sql

# Restore from compressed
gunzip < db.sql.gz | mysql -u root -p <database>

# Import CSV
LOAD DATA INFILE '/path/file.csv'
INTO TABLE <table>
FIELDS TERMINATED BY ','
ENCLOSED BY '"'
LINES TERMINATED BY '\n'
IGNORE 1 ROWS;

# Export to CSV
SELECT * FROM <table>
INTO OUTFILE '/tmp/output.csv'
FIELDS TERMINATED BY ','
ENCLOSED BY '"'
LINES TERMINATED BY '\n';
```

## Configuration (my.cnf)

```ini
[mysqld]
# Storage engine
default-storage-engine=INNODB

# Character set
character-set-server=utf8mb4
collation-server=utf8mb4_unicode_ci

# Connection
max_connections=200
wait_timeout=600
interactive_timeout=600

# Query cache (deprecated in MySQL 8+)
# query_cache_type=0

# InnoDB
innodb_buffer_pool_size=1G        # 70-80% of RAM for dedicated DB
innodb_log_file_size=256M
innodb_flush_log_at_trx_commit=2  # 1=ACID, 2=faster
innodb_file_per_table=1

# Logging
slow_query_log=1
slow_query_log_file=/var/log/mysql/slow.log
long_query_time=2
log_error=/var/log/mysql/error.log

# Binary log (for replication / point-in-time recovery)
# server_id=1
# log_bin=/var/log/mysql/binlog
# binlog_format=ROW
# expire_logs_days=7
```

## Common Fixes

```sql
-- Table is corrupted
CHECK TABLE <table>;
REPAIR TABLE <table>;

-- Can't connect: too many connections
-- Connect as root (always reserved connection)
mysql -u root -p -e "SHOW VARIABLES LIKE 'max_connections';"
mysql -u root -p -e "SET GLOBAL max_connections = 500;"

-- Reset root password (start with --skip-grant-tables)
-- 1. Stop MySQL
-- 2. mysqld_safe --skip-grant-tables &
-- 3. mysql -u root
-- 4. FLUSH PRIVILEGES;
-- 5. ALTER USER 'root'@'localhost' IDENTIFIED BY '<newpass>';

-- InnoDB corruption
-- Set in my.cnf: innodb_force_recovery=1..6 (start with 1)
-- Dump what you can, restore from backup

-- Disk full / table too big
-- Optimize table (rebuilds index, frees space)
OPTIMIZE TABLE <table>;
```

## Important Information Schema Tables

```sql
-- All tables with metadata
information_schema.TABLES

-- All columns
information_schema.COLUMNS

-- All indexes
information_schema.STATISTICS

-- All processes
information_schema.PROCESSLIST

-- InnoDB status
information_schema.INNODB_METRICS
information_schema.INNODB_LOCKS
information_schema.INNODB_LOCK_WAITS
```

## Important Files

| Path | Purpose |
|------|---------|
| `/etc/mysql/my.cnf` | Main MySQL config |
| `/etc/mysql/mysql.conf.d/mysqld.cnf` | Ubuntu: daemon config |
| `/var/lib/mysql/` | Data directory |
| `/var/log/mysql/error.log` | Error log |
| `/var/log/mysql/slow.log` | Slow query log |
| `~/.my.cnf` | User credentials (avoid password on CLI) |
