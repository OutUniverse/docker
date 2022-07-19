FROM docker.elastic.co/elasticsearch/elasticsearch:7.5.2

RUN cd /usr/share/elasticsearch/bin \
    && elasticsearch-plugin install --batch analysis-icu \
    && elasticsearch-plugin install --batch https://github.com/medcl/elasticsearch-analysis-ik/releases/download/v7.5.2/elasticsearch-analysis-ik-7.5.2.zip
