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

