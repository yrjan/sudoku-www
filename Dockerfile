FROM nginx:alpine
COPY index.html styles.css /usr/share/nginx/html/
COPY pkg /usr/share/nginx/html/pkg
RUN ls /usr/share/nginx/html
