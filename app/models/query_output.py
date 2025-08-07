from pydantic import BaseModel
from typing import List, Dict

class QueryOutput(BaseModel):
    columns: List[str]
    rows: List[Dict[str, str]]
