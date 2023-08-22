from fastapi import FastAPI
from fastapi import status, Form
from fastapi.param_functions import Depends
from fastapi.staticfiles import StaticFiles
from pydantic import BaseModel
from pydantic.errors import FrozenSetError
from starlette.responses import RedirectResponse
from fastapi.security import OAuth2PasswordBearer
from db import *
from BaseModels import *
class Login(BaseModel):

    username: str
    password: str

app = FastAPI()
app.mount("/ui", StaticFiles(directory = "ui"), name = "ui")

@app.get("/")
def get_home():

    return RedirectResponse("/ui/login.html", status.HTTP_302_FOUND)

@app.get("/index")
def get_index():

    return RedirectResponse("/ui/index.html", status.HTTP_302_FOUND)

@app.post("/create_tables")
def post_create_tables():

    create_tables()

@app.post("/check_login")
def check_login(login: Login):

    if (login.username == "Management" and login.password == "System"):

        return "Correct"
    else:

        return "WRONG"

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
async def post_add_existing_item(item: Item1):
    
    return add_exisiting_item(item.id, item.amount)
    

@app.post("/withdraw_item")
def post_withdraw_item(item: Item1):

    return withdraw_item(item.id, item.amount)

@app.post("/remove_item")
def post_remove_item(item: Item2):
    
    return remove_item(item.id)

@app.post("/change_location")
def post_change_location(item: Item3):

    return change_location_item(item.id, item.location)

@app.post("/search_by_name")
def post_search_by_name(name: str):

    return get_by_name(name)

@app.post("/search_by_id")
def post_search_by_id(id: str):

    return get_by_id(id)