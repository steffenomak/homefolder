# Lines configured by zsh-newuser-install
HISTFILE=~/.histfile
HISTSIZE=1000
SAVEHIST=1000
setopt autocd extendedglob
unsetopt beep
bindkey -e
# End of lines configured by zsh-newuser-install
# The following lines were added by compinstall
zstyle :compinstall filename '/home/steffenomak/.zshrc'

autoload -Uz compinit
compinit
# End of lines added by compinstall

alias ls='ls --color=auto'
alias e='emacsclient'
alias pac='sudo pacman-color -S'
alias pacu='yaourt -Syua'
alias pacs='pacman-color -Ss'
alias pacui='sudo pacman-color -Rd'
alias greps='grep -inIEr --color=ALWAYS'
alias pacman='pacman-color'

alias pm-suspend='sudo pm-suspend'

export PATH=$PATH:/opt/mosml/bin:$HOME/scripts:/opt/google/talkplugin:/opt/android-sdk/platform-tools:/opt/android-sdk/tools

export EDITOR="emacs -nw"

export _humblebundle4key=tXqmP2KznG8k
export R600_ENABLE_S3TC=1

u_1='┌─'
u_2='└──■'
u_3='■──■'

r_1='┌─'
r_2='└──>'
r_3='<──>'

if [ `id -u` -eq "0" ]; then
	i_1=${r_1}
	i_2=${r_2}
	i_3=${r_3}
else
	i_1=${u_1}
	i_2=${u_2}
	i_3=${u_3}
fi

setopt PROMPT_SUBST

source ~/.git-prompt.sh

PROMPT=${i_1}$'[\e[0;35m%n\e[0m@%m %c''$(__git_ps1 " (\e[1;33m%s\e[0m)")'$'] \e[0;35m$\e[0m\n'${i_2}' '
PS2=${i_3}' '

export GIT_PS1_SHOWDIRTYSTATE=true
export GIT_PS1_SHOWUNTRACKEDFILES=true
export GIT_PS1_SHOWSTASHSTATE=true

zstyle ":completion:*:commands" rehash 1
