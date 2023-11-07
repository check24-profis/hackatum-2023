## Simple App with Node Server and Postgres Database

Please ensure docker is installed.

#### What does this App provide?

It provides a simple user interface with some input-fields where user can:

1. Query craftsmen by postalcode.
2. Change each craftman's ranking by id.



#### How to start the app

1. To start `docker-compose up [-d]`
2. To stop `docker-compose stop`
3. And then open http://localhost:3000/ in browser.



#### How to start for developing:

1. Ensure you installed node(>=18).

2. Run `npm install`

3. Ensure database docker container is running, you can build the image & run the container with following commands:

   ```bash
   $ docker build . -t dev_postgres -f Dockerfile.database.dev
   $ docker run --name dev_postgres_container -d -p 5432:5432 dev_postgres
   ```

4. Then run `npm start`

