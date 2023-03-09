use rustpython_vm::{
    compiler, pyclass, pymodule, PyObject, PyPayload, PyResult, TryFromBorrowedObject,
    VirtualMachine,
};
use std::fs::File;
use std::io::Read;
use std::process::Command;

pub fn main() {
    let interp = rustpython::InterpreterConfig::new()
        .init_stdlib()
        .init_hook(Box::new(|vm| {
            vm.add_native_module("aruba".to_owned(), Box::new(aruba::make_module));
        }))
        .interpreter();

    interp.enter(|vm| {
        let scope = vm.new_scope_with_builtins();

        let mut file = File::open("src/example_script.py").unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let code_obj = vm
            .compile(&contents, compiler::Mode::Exec, "<embedded>".to_owned())
            .map_err(|err| vm.new_syntax_error(&err))
            .unwrap();

        vm.run_code_obj(code_obj, scope).unwrap();
    })
}

#[pymodule]
mod aruba {
    use super::*;
    use rustpython::vm::{builtins::PyList, convert::ToPyObject, PyObjectRef};

    #[pyfunction]
    fn ping(c: i64, url: String, _vm: &VirtualMachine) -> PyResult<String> {
        let output = Command::new("ping")
            .args(["-c", &c.to_string(), &url])
            .output()
            .expect("failed to execute process");

        let result = match std::str::from_utf8(&output.stdout) {
            Ok(r) => r,
            Err(_) => "",
        };

        Ok(result.to_string())
    }

    #[pyfunction]
    fn send(data: String, _vm: &VirtualMachine) -> PyResult<()> {
        // this is just some function to mimic sending the data to the server
        println!("{:?}", data);

        Ok(())
    }

    // below is just a whole bunch of utility stuff I found such as traits for converting
    // rust structs to python objects and so on.

    // #[derive(Debug, Clone)]
    // struct NumVec(Vec<i32>);

    // impl ToPyObject for NumVec {
    //     fn to_pyobject(self, vm: &VirtualMachine) -> PyObjectRef {
    //         let list = self.0.into_iter().map(|e| vm.new_pyobj(e)).collect();
    //         PyList::new_ref(list, vm.as_ref()).to_pyobject(vm)
    //     }
    // }

    // #[pyattr]
    // #[pyclass(module = "rust_py_module", name = "RustStruct")]
    // #[derive(Debug, PyPayload)]
    // struct RustStruct {
    //     numbers: NumVec,
    // }

    // #[pyclass]
    // impl RustStruct {
    //     #[pygetset]
    //     fn numbers(&self) -> NumVec {
    //         self.numbers.clone()
    //     }

    //     #[pymethod]
    //     fn print_in_rust_from_python(&self) {
    //         println!("Calling a rust method from python");
    //     }
    // }

    // struct PythonPerson {
    //     name: String,
    // }

    // impl TryFromBorrowedObject for PythonPerson {
    //     fn try_from_borrowed_object(vm: &VirtualMachine, obj: &PyObject) -> PyResult<Self> {
    //         let name = obj.get_attr("name", vm)?.try_into_value::<String>(vm)?;
    //         Ok(PythonPerson { name })
    //     }
    // }
}
