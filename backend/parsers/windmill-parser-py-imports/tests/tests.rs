mod tests {

    use sqlx::{Pool, Postgres};
    use windmill_parser_py_imports::parse_python_imports;

    #[sqlx::test(fixtures("base"))]
    async fn test_parse_python_imports(db: Pool<Postgres>) -> anyhow::Result<()> {
        //let code = "print(2 + 3, fd=sys.stderr)";
        let code = "

import os
import wmill
from zanzibar.estonie import talin
import matplotlib.pyplot as plt
from . import tests

def main():
    pass

";
        let r = parse_python_imports(code, "test-workspace", "f/foo/bar", &db).await?;
        // println!("{}", serde_json::to_string(&r)?);
        assert_eq!(r, vec!["matplotlib", "requests", "wmill", "zanzibar"]);
        Ok(())
    }

    #[sqlx::test(fixtures("base"))]
    async fn test_parse_python_imports2(db: Pool<Postgres>) -> anyhow::Result<()> {
        //let code = "print(2 + 3, fd=sys.stderr)";
        let code = "
#requirements:
#burkina=0.4
#nigeria
#
#congo

import os
import wmill
from zanzibar.estonie import talin

def main():
    pass

";
        let r = parse_python_imports(code, "test-workspace", "f/foo/bar", &db).await?;
        println!("{}", serde_json::to_string(&r)?);
        assert_eq!(r, vec!["burkina=0.4", "nigeria"]);

        Ok(())
    }

    #[sqlx::test(fixtures("base"))]
    async fn test_parse_python_imports_local(db: Pool<Postgres>) -> anyhow::Result<()> {
        //let code = "print(2 + 3, fd=sys.stderr)";
        let code = "
from f.foo.bar import main1
from .baz import main2
from ..foo.bar import main3
import f.foo.bar as bar


def main():
    pass

";
        let r = parse_python_imports(code, "test-workspace", "f/foo/bar", &db).await?;
        println!("{}", serde_json::to_string(&r)?);
        assert_eq!(r, ["numpy", "pandas", "pandas2", "requests"]);

        Ok(())
    }
}
