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
    amout: str
    id: str
    location: str