sudo docker build -t frontend:latest ./services/frontend
sudo docker build -t greeter:latest ./services/greeter
sudo docker build -t streamer2:latest ./services/streamer2
sudo docker build -t task_dispatch:latest ./services/task_dispatch
sudo docker build -t translator:latest ./services/translator
sudo kubectl apply -f k8s/monitoring
sudo kubectl apply -f k8s/services
