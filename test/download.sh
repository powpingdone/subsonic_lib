#!/bin/bash

wget http://www.subsonic.org/pages/api.jsp -r -A .xml
mv www.subsonic.org/pages/inc/api/examples/* .
rm -r www.subsonic.org
