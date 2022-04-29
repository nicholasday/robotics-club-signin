import getpass
from fabric import Connection, Config, task

def sudo():
    sudo_pass = getpass.getpass("What's your sudo password? ")
    config = Config(overrides={'sudo': {'password': sudo_pass}})
    return config

@task
def build_backend(c):
    c.local("cargo build --release --target x86_64-unknown-linux-musl")
    c.local("cp target/x86_64-unknown-linux-musl/release/robotics-club-signin ./")
    c.local("tar cvzf build.tar.gz .env.prod Rocket.toml robotics-club-signin") 

@task
def deploy_backend(c):
    c.config = sudo()
    build_backend(c)
    c.put("build.tar.gz", remote="/home/nicholas/")
    c.sudo("tar xvf build.tar.gz -C /var/www/robotics-club-signin/")
    c.sudo("mv /var/www/robotics-club-signin/.env.prod /var/www/robotics-club-signin/.env")
    c.sudo("rm build.tar.gz")
    c.sudo("chown -R www-data /var/www/robotics-club-signin")
    c.sudo("chmod 700 -R /var/www/robotics-club-signin")
    c.sudo("chmod +x /var/www/robotics-club-signin/robotics-club-signin")
    c.sudo("systemctl restart robotics-club-signin")
    clean_backend(c)

@task
def clean_backend(c):
    c.local("rm robotics-club-signin build.tar.gz")

@task
def build_frontend(c):
    with c.cd("frontend"):
        c.local("yarn build")
        c.local("tar cvzf frontend.tar.gz dist")
        c.local("mv frontend.tar.gz ../")

@task
def clean_frontend(c):
    c.local("rm frontend.tar.gz")

@task
def deploy_frontend(c):
    c.config = sudo()
    build_frontend(c)
    c.put("frontend.tar.gz", remote="/home/nicholas/")
    c.sudo("tar xvf frontend.tar.gz -C /var/www/robotics-club-signin/")
    c.sudo("rm frontend.tar.gz")
    c.sudo("chown -R www-data /var/www/robotics-club-signin")
    c.sudo("chmod 700 -R /var/www/robotics-club-signin")
    clean_frontend(c)

@task
def deploy(c):
    deploy_backend(c)
    deploy_frontend(c)
