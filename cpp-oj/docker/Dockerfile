FROM archlinux/archlinux
# 必要なパッケージをインストール
RUN pacman -Syu --noconfirm \
&&  pacman -S --noconfirm base-devel git unzip gcc12 clang16 pkg-config sudo \
    curl tmux zsh fzf zoxide rustup fd ripgrep bat lsd neovim tig make \
    python-pipx boost gmp eigen

# ac-libraryをインストール
RUN mkdir -p /opt/ac-library \
&&  git clone https://github.com/atcoder/ac-library /opt/ac-library \
&&  cd /opt/ac-library \
&&  git checkout production

# 一般ユーザarchを作って切り替える
ARG user_name=arch
RUN useradd ${user_name} -d /home/${user_name} --create-home \
&&  echo "${user_name} ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers \
&&  chown -R ${user_name}:${user_name} /home/${user_name} \
&&  chsh -s /bin/zsh ${user_name}
USER ${user_name}
ENV HOME /home/${user_name}

# nodeとpythonのインストール
ENV PATH $PATH:$HOME/.cargo/bin
RUN rustup install stable \
&&  cargo install mise \
&&  mise install node --yes \
&&  sudo mise use node --yes \
&&  mise install python@3.10 --yes \
&&  sudo mise use python@3.10 --yes

# ojのインストール
# python3.12だとうまく動かないので3.10を使う
RUN zsh -c "pipx install online-judge-tools --python $(mise where python@3.10)/bin/python3"

# dotfilesの設置
RUN mkdir -p $HOME/repos/ \
&&  git clone https://github.com/yuma140902/dotfiles-public $HOME/repos/dotfiles-public \
&&  cd $HOME/repos/dotfiles-public/ \
&&  ./dotfiles install zsh \
&&  ./dotfiles install neovim \
&&  ./dotfiles install tmux

# neovimのプラグインをインストールする
RUN nvim -c "Lazy restore" -c "qa!" \
&&  nvim -c "MasonInstall clangd typos-lsp" -c "qa!"

# tmuxのプラグインをインストールする
ENV TMUX_PLUGIN_MANAGER_PATH=$HOME/.tmux/plugins/tpm
ENV TERM=xterm
RUN git clone https://github.com/tmux-plugins/tpm $HOME/.tmux/plugins/tpm \
&&  zsh -c "$HOME/.tmux/plugins/tpm/scripts/install_plugins.sh ; exit 0"

# zinitのプラグインをインストールする
RUN /bin/zsh -c "source $HOME/.zshrc"

# atcoderリポジトリのクローン
RUN git clone https://github.com/yuma140902/atcoder $HOME/atcoder
WORKDIR $HOME/atcoder/

COPY scripts.zsh $HOME/scripts.zsh

RUN echo "export TERM=screen" >> "$HOME/.zshrc"
RUN echo "[ -f ~/scripts.zsh ] && source ~/scripts.zsh" >> "$HOME/.zshrc"
CMD tmux
