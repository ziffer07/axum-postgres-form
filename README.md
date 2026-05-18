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

The Data stored in postgres locally is now accessible on the front page. The process is as follows:

1. You fill the form with name and email.

2. Data is stored in postgres (psql) using PgPool and add_name_email_to_db query. Upon submitting the user is redirected to the index.html.

3. Then using get_all_data we query the database to get all elements of the form using name and email in descending order by their id.

4. The sql error is handled using AppError enum.

5. Each element is then displayed in index.html


I have added a few more columns of title and description in the table. However, I couldn't just run the migration again. Therefore, I had to drop the database and then create new database again and then run sqlx migrations to add table. Then, I had to include the filed that I added in the form-page and update main.rs with title and description. I also had to update pgpool because my instance was somehow running to had to add some commands to cancel the connection after a while.


### Next Updates
1. Add logging and tracing features to the code

2. Seperate the main.rs into files where we have handler file, template file.

3. Update Error Handling and handle all edge cases

4. Update more field and try to make changes happen without too much hassle like above.