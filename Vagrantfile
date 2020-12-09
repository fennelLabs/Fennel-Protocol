# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
  config.vm.box = "bento/ubuntu-20.04"

  config.vm.network "private_network", ip: "192.168.33.11"
  config.vm.network :forwarded_port, guest: 22, host: 2222, id: 'ssh'

  config.vm.synced_folder ".", "/home/vagrant/femr_onchain"

  config.vm.provider "virtualbox" do |vb|
    vb.gui = false
    vb.memory = "1024"
  end

  config.vm.provision "shell", inline: <<-SHELL
    apt-get clean
    apt-get update
    apt-get upgrade -y
  SHELL
end
