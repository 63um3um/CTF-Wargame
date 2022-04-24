#FROM ubuntu:18.04
FROM ubuntu@sha256:9bc830af2bef73276515a29aa896eedfa7bdf4bdbc5c1063b4c457a4bbb8cd79

# Setup environ
ENV user oxidized
ENV prob_port 13100

# Install packages
RUN apt-get update
RUN apt-get install -y socat

# Change permission
RUN chmod 1733 /tmp /var/tmp /dev/shm

# Additional configuration
RUN adduser $user
ADD ./chal /home/$user/chal
ADD ./flag /home/$user/flag

RUN chown -R root:root /home/$user/
RUN chown root:$user /home/$user/chal
RUN chown root:$user /home/$user/flag

RUN chmod 2755 /home/$user/chal
RUN chmod 440 /home/$user/flag

# final
WORKDIR /home/$user
CMD socat -T 60 TCP-LISTEN:$prob_port,reuseaddr,fork EXEC:/home/$user/chal
USER $user
EXPOSE $prob_port
