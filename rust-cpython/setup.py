import setuptools_rust
from setuptools import setup
setup(name='librustpython', rust_extensions=[setuptools_rust.RustExtension('.', binding=setuptools_rust.Binding.RustCPython)], zip_safe=False)

