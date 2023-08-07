from pydantic import BaseModel
from typing import Optional
import os
from passlib.context import CryptContext
from datetime import date, datetime, timedelta
import random
import sqlite3 as driver
from sqlite3.dbapi2 import Cursor

DATABASE_URL = 'db/DTItems.db'

class Item(BaseModel):

    name: str
    amount: str
    id: str
    location: str

def create_tables():

    database = driver.connect(DATABASE_URL)
    cursor = database.cursor()
    cursor.execute("CREATE TABLE IF NOT EXISTS items (name TEXT, amount TEXT, id TEXT, location TEXT);")

def clear_tables():

    database = driver.connect(DATABASE_URL)
    cursor = database.cursor()
    cursor.execute("DELETE FROM items")
    database.commit()

def list_items():

    database = driver.connect(DATABASE_URL)
    cursor = database.cursor()
    items = cursor.execute("SELECT * FROM  items;")
    items = items.fetchall()
    arr = []
    for item in items:

        arr.append(Item(name=item[0], amount=item[1], id=item[2], location=item[3]))

    return arr

def add_item(name: str, amount: str, id: str, location: str):

    database = driver.connect(DATABASE_URL)
    cursor = database.cursor()
    cursor.execute(f"INSERT INTO items (name, amount, id, location) VALUES ('{name}', '{amount}', '{id}', '{location}');")
    database.commit()


def add_exisiting_item(id: str, amount: str):

    database = driver.connect(DATABASE_URL)
    cursor = database.cursor()
    res = cursor.execute(f"SELECT * FROM items WHERE id='{id}';")
    res = res.fetchall()
    if res == []:
     return f"Could not find item with the ID {id}"
    else:

        cursor.execute(f"UPDATE items SET amount='{str(int(res[0][1]) + int(amount))}' WHERE id='{id}';")
        database.commit()
    
    return "Successful"

def withdraw_item(id: str, amount: str):

    database = driver.connect(DATABASE_URL)
    cursor = database.cursor()
    res = cursor.execute(f"SELECT * FROM items WHERE id='{id}';")
    res = res.fetchall()
    if res == []:

        return f"Could not find item with id: {id}"
    else:
        if int(amount) > int(res[0][1]):

            return "There is not enough of the item to remove, nothing has been removed as a result"
        else:
            
            cursor.execute(f"UPDATE items SET amount='{str(int(res[0][1]) - int(amount))}' WHERE id='{id}';")
            database.commit()
            return "Successful"
 
def remove_item(id: str):

    database = driver.connect(DATABASE_URL)
    cursor = database.cursor()
    cursor.execute(f"DELETE FROM items WHERE id='{id}';")
    database.commit()

def change_location_item(id: str, location: str):

    database = driver.connect(DATABASE_URL)
    cursor = database.cursor()
    res = cursor.execute(f"SELECT * FROM items WHERE id='{id}';")
    res = res.fetchall()
    if res == []:

        return f"Could not find item with id: {id}"
    else:
            
        cursor.execute(f"UPDATE items SET location='{location}' WHERE id='{id}';")
        database.commit()
        return "Successful"

