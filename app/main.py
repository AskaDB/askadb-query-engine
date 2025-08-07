
from fastapi import FastAPI
from app.routers import execute

app = FastAPI(title="askadb - Query Engine")

app.include_router(execute.router, prefix="/execute", tags=["Query Execution"])
