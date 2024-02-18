use gtk4::StringList;
use pyo3::prelude::*;
use pyo3::types::{PyTuple, IntoPyDict};
use std::collections::HashMap;


fn main() -> PyResult<()> {
    let _script: &str = "def req_data(*args, **kwargs):
    import requests
    _url = kwargs.get('url')
    s = requests.get(_url)
    return s.text
";

    Python::with_gil(|py| {
        let mut _params = HashMap::new();
        _params.insert("url", "https://www.baidu.com");
        let _ts = _params.into_py_dict(py);
        let reqnetwork = PyModule::from_code(
            py, _script, "", "").unwrap();
        let _res_str: String = reqnetwork
                .getattr("req_data").unwrap()
                .call((), Some(_ts)).unwrap().extract().unwrap();

        
        // println!("{:#?}", _ts);
        println!("request result: {}", _res_str);
    });

    
    Ok(())
}


fn main_() -> PyResult<()> {
    let arg1 = "arg1";
    let arg2 = "arg2";
    let arg3 = "arg3";

    Python::with_gil(|py| {
        let fun: Py<PyAny> = PyModule::from_code(
            py,
            "def example(*args, **kwargs):
                if args != ():
                    print('called with args', args)
                if kwargs != {}:
                    print('called with kwargs', kwargs)
                if args == () and kwargs == {}:
                    print('called with no arguments')",
            "",
            "",
        )?
        .getattr("example")?
        .into();

        // call object without any arguments
        fun.call0(py)?;

        // call object with PyTuple
        let args = PyTuple::new(py, &[arg1, arg2, arg3]);
        fun.call1(py, args)?;

        // pass arguments as rust tuple
        let args = (arg1, arg2, arg3);
        fun.call1(py, args)?;
        Ok(())
    })
}
