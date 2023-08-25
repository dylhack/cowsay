docker run -p 5672:5672 -p 15672:15672 -e RABBITMQ_DEFAULT_VHOST=cowsay -e RABBITMQ_DEFAULT_USER=rabbit -e RABBITMQ_DEFAULT_PASS=rabbit -it rabbitmq:3-management

