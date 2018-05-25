### Ubuntu 18.04 install docker CE
官方英文原版教程地址：https://docs.docker.com/install/linux/docker-ce/ubuntu/

卸载老版本：
```
sudo apt-get remove docker docker-engine docker.io
```
安装依赖库：
```
sudo apt-get install apt-transport-https ca-certificates curl software-properties-common
```
添加gpgkey：
```
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
```

验证添加的gpgkey：
```
sudo apt-key fingerprint 0EBFCD88
```
正常情况下，命令行会输出如下信息：
```
pub   rsa4096 2017-02-22 [SCEA]
      9DC8 5822 9FC7 DD38 854A  E2D8 8D81 803C 0EBF CD88
uid           [ 未知 ] Docker Release (CE deb) <docker@docker.com>
sub   rsa4096 2017-02-22 [S]
```
安装stable仓库：
> 由于Ubuntu 18.04还没有stable版本，所以暂时只能安装test版本，等官方弄好后，在安装stable版本，[详见Github issues](https://github.com/docker/for-linux/issues/290)
```
sudo add-apt-repository \
   "deb [arch=amd64] https://download.docker.com/linux/ubuntu \
   $(lsb_release -cs) \
   test"
```
安装docker：
```
sudo apt-get update
sudo apt-get install docker-ce
```
验证是否安装成功：
```
sudo docker run hello-world
```
看到如下消息，代表安装成功：
```
Hello from Docker!
This message shows that your installation appears to be working correctly.
```