from setuptools import setup, find_packages
from setuptools_rust import RustExtension

setup(
    name="optimaimg",
    version="0.1.0",
    author="Hamsa Jama",
    author_email="hamsa.a.j@gmail.com",
    description="An image processing toolkit combining Python and Rust",
    long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
    url="https://github.com/PrimeCodeLabs/optimaimg",
    packages=find_packages(),
    rust_extensions=[
        RustExtension("optimaimg.optimaimg_rust", "Cargo.toml", binding="pyo3")
    ],
    include_package_data=True,
    zip_safe=False,
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
)
