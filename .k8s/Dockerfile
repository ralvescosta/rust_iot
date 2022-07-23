FROM eclipse-mosquitto

COPY ./mosquitto.conf /mosquitto/config/mosquitto.conf
COPY ./mosquitto_password.txt /mosquitto/config/mosquitto_password.txt

RUN mosquitto_passwd -U /mosquitto/config/mosquitto_password.txt

CMD ["/usr/sbin/mosquitto", "-c", "/mosquitto/config/mosquitto.conf"]