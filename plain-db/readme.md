
`docker run --name mysql-server -e MYSQL_ROOT_PASSWORD=123456 -d -p 3306:3306 mysql:latest`

`docker exec -it mysql-server mysql -u root -p`

`CREATE DATABASE test;`