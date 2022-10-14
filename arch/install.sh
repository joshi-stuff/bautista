## arg 1:  the new package version
#pre_install() {
	# do something here
#}

## arg 1:  the new package version
post_install() {
	echo useradd -r -s /usr/bin/nologin bautista -U
}

## arg 1:  the new package version
## arg 2:  the old package version
#pre_upgrade() {
	# do something here
#}

## arg 1:  the new package version
## arg 2:  the old package version
#post_upgrade() {
	# do something here
#}

## arg 1:  the old package version
pre_remove() {
	echo userdel -r bautista -U
}

## arg 1:  the old package version
#post_remove() {
	# do something here
#}
