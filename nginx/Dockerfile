FROM nginx:alpine
COPY nginx/nginx.conf /etc/nginx/nginx.conf
COPY nginx/default.conf /etc/nginx/conf.d/default.conf
COPY nginx/cert/private.key /etc/cert/private.key
COPY nginx/cert/certificate.crt /etc/cert/certificate.crt
COPY nginx/cert/passwords /etc/cert/passwords
EXPOSE 80 443 8080
