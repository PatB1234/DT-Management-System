from fastapi import FastAPI
from fastapi import status, Form
from fastapi.param_functions import Depends
from fastapi.staticfiles import StaticFiles
from pydantic import BaseModel
from pydantic.errors import FrozenSetError
from starlette.responses import RedirectResponse
from fastapi.security import OAuth2PasswordBearer
from db import *

app = FastAPI()
app.mount("/ui", StaticFiles(directory = "ui"), name = "ui")

@app.get("/")
def get_home():

    return RedirectResponse("/ui/index.html", status.HTTP_302_FOUND)

@app.post("/create_tables")
def post_create_tables():

    create_tables()

@app.post("/clear_tables")
def post_clear_tables():

    clear_tables()

@app.get("/items")
def get_items():

    return list_items()

@app.post("/add_items")
def post_add_items(name: str = Form(...), amount: str = Form(...), id: str = Form(...) , location: str = Form(...)):

    add_item(name, amount, id, location)
    return RedirectResponse("/ui/AddItem.html", status.HTTP_302_FOUND)

@app.post("/add_existing_item")
def post_add_exisiting_item(id: str, amount: str):

    return add_exisiting_item(id, amount)

@app.post("/withdraw_item")
def post_withdraw_item(id: str, amount: str):

    return withdraw_item(id, amount)

@app.post("/remove_item")
def post_remove_item(id: str):
    
    return remove_item(id)

@app.post("/change_location")
def post_change_location(id: str, location: str):

    return change_location_item(id, location)