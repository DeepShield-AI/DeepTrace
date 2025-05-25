cd ./server
source venv/bin/activate
cd controller
python3 cmd.py stop
cd ../database
python3 clear.py
cd ..
rm -rf venv