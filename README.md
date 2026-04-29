The code is updated form from the previous axum-basics repository 

Here, I have included 2 pages in the templates, home page and form page. The navigation bar is used to go to form page.

On the form page, we are taking the input name and input email and then displaying that on the form-page itself.

The data submitted from the form is stored in postgres sql and is stored in the contents table of the database.

For this to work, you need to have installed postgres locally:

The you can run the following commands:

            sudo -i -u postgres
            psql

When you are in psql, you can set the password for the username mentioned in your .env file.

After that you have to run the following commands

            sqlx database create
            sqlx migrate add <table name>

First script above create your database, the name of the db is as mentioned in your URL in the .env file. You can check if your db is created with ```\l``` in your terminal

The second script, creates a folder called migrate with the table name in your project folder at the level of src folder. In this file you write your
migration scripts like CREATE TABLE, etc and give all the columns and parameters

This is all the setup needed.

For running the code, make sure your table name, database connections, etc are named similar to what I have.

To run the project you can do as follows:

            cargo run

The localhost http address will be in the printed in the terminal, you can copy that in your browser and run the code.


### Next Updates
1. Add logging and tracing features to the code

2. Seperate the main.rs into files where we have handler file, template file.

3. Use the data stored in postgres and query it on the front end.

4. Update Error Handling and handle all edge cases