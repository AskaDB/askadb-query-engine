from fastapi import APIRouter, HTTPException
from app.models.query_input import QueryInput
from app.models.query_output import QueryOutput
from app.services.engine import run_query

router = APIRouter()

@router.post("/", response_model=QueryOutput)
async def execute_query(input: QueryInput):
    try:
        result = run_query(input.query)
        return QueryOutput(**result)
    except Exception as e:
        raise HTTPException(status_code=400, detail=str(e))
