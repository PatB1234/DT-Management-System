from pydantic import BaseModel
from typing import Optional
from passlib.context import CryptContext
from datetime import date, datetime, timedelta
import random
import sqlite3 as driver
from sqlite3.dbapi2 import Cursor
import os
import urllib.parse as up
import psycopg2
up.uses_netloc.append("postgres")
url = up.urlparse(os.environ["DATABASE_URL"])
conn = psycopg2.connect(database=url.path[1:],
user=url.username,
password=url.password,
host=url.hostname,
port=url.port
)
class Item(BaseModel):

    name: str
    amount: str
    id: str
    location: str

class Item1(BaseModel):

    id: str
    amount: str    
class Item2(BaseModel):

    id: str

class Item3(BaseModel):

    id: str
    location: str
def create_tables():

    cursor = conn.cursor()
    cursor.execute("CREATE TABLE IF NOT EXISTS items (name TEXT, amount TEXT, id TEXT, location TEXT);")
    conn.commit()

def clear_tables():

    cursor = conn.cursor()
    cursor.execute("DELETE FROM items")
    conn.commit()

def list_items():

    cursor = conn.cursor()
    items = cursor.execute("SELECT * FROM  items;")
    items = items.fetchall()
    arr = []
    for item in items:

        arr.append(Item(name=item[0], amount=item[1], id=item[2], location=item[3]))

    return arr

def add_item(name: str, amount: str, id: str, location: str):

    cursor = conn.cursor()
    cursor.execute(f"INSERT INTO items (name, amount, id, location) VALUES ('{name}', '{amount}', '{id}', '{location}');")
    conn.commit()


def add_exisiting_item(id: str, amount: str):

    cursor = conn.cursor()
    res = cursor.execute(f"SELECT * FROM items WHERE id='{id}';")
    res = res.fetchall()
    if res == []:
     return f"Could not find item with the ID {id}"
    else:
        cursor.execute(f"UPDATE items SET amount='{str(int(res[0][1]) + int(amount))}' WHERE id='{id}';")
        conn.commit()
    
    return "Successful"

def withdraw_item(id: str, amount: str):

    cursor = conn.cursor()
    res = cursor.execute(f"SELECT * FROM items WHERE id='{id}';")
    res = res.fetchall()
    if res == []:

        return f"Could not find item with id: {id}"
    else:
        if int(amount) > int(res[0][1]):

            return "There is not enough of the item to remove, nothing has been removed as a result"
        else:
            
            cursor.execute(f"UPDATE items SET amount='{str(int(res[0][1]) - int(amount))}' WHERE id='{id}';")
            conn.commit()
            return "Successful"
 
def remove_item(id: str):

    cursor = conn.cursor()
    res = cursor.execute(f"SELECT * FROM items WHERE id='{id}';")
    res = res.fetchall()
    if res == []:

        return f"Could not find item with id: {id}"
    else:
        cursor.execute(f"DELETE FROM items WHERE id='{id}';")
        conn.commit()
        return "Successful"

def change_location_item(id: str, location: str):

    cursor = conn.cursor()
    res = cursor.execute(f"SELECT * FROM items WHERE id='{id}';")
    res = res.fetchall()
    if res == []:

        return f"Could not find item with id: {id}"
    else:
            
        cursor.execute(f"UPDATE items SET location='{location}' WHERE id='{id}';")
        conn.commit()
        return "Successful"

