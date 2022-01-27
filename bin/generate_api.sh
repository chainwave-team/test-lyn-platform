#!/bin/bash

echo " ";
echo "### Endpoints";
echo " ";
rg -No --engine="pcre2" -e "(?<=endpoint\()(.*)(?=\))" ./src/lib.rs | sed 's/.*/- `&()`/'; 
echo " ";
echo "### View";
echo " ";
rg -No --engine="pcre2" -e "(?<=view\()(.*)(?=\))" ./src/lib.rs | sed 's/.*/- `&()`/'; 
echo " ";
echo "### Storage"; 
echo " ";
rg -No --engine="pcre2" -e "(?<=storage_mapper\(\")(.*)(?=\"\))" ./src/lib.rs | sed 's/.*/- `&`/';