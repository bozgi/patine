# E-mail server software written in Rust

This is a work in progress implementation of RFC 5321 
and some other standards that make the current E-mail 
safe and usable. It should not be used in production at 
this point.

# Installation and setup

For now this software works only on Linux. To build it you 
need to have the [Rust toolchain](https://www.rust-lang.org/tools/install)
installed, as well as PAM development libraries. To build it run

```bash
cargo build --release
cargo build --bin pam_helper --release
```

Create user `patine` and group `mailwriters`. Add `patine` to 
`mailwriters`. In your `sudoers` file add the following line:

```
patine ALL=(ALL) NOPASSWD: /absolute/location/to/pam_helper
```

this will allow Patine to authenticate using PAM.

## Certificates

To run this program you will need to get TLS certificates.
You can use [Let's Encrypt](https://letsencrypt.org/getting-started/).
Place the certificates in `certs` folder located at the same level as
the Patine binary. Name the certificate `cert.pem` and the private 
key `cert.key.pem`. 

## Environment variables

There are 5 required environment variables you will need to set:

1. `MAILDIR_ROOT` - Where each user's mails are stored
2. `DOMAIN` - The domain of the mail 
3. `RELAY_PORT` - Relaying port (Strongly recommend 25)
4. `SUBMISSION_PORT` - Submission port (Strongly recommend 587)
5. `PAM_HELPER_PATH` - Absolute location of the `pam_helper`

## PAM Helper

Patine utilises PAM (**P**luggable **A**uthentication **M**odules).
That means that the AUTH command authenticates submission mail if
the supplied login and password can be used to log in to a Linux
account.

## Creating and removing users

The `create-mailbox.sh`and `remove-mailbox.sh` will create 
or remove the user with the supplied name and password, 
as well as create the mail directory in the 
Maildir standard. This defaults to `/var/mail/$username`.

## Running the app

Once all setup is complete run

```
tmux
sudo -u patine ./patine
```

Remember to allow `SUBMISSION_PORT` and `RELAY_PORT` in
your firewall.

# Reading the mail

I recommend using [Dovecot](https://dovecot.org). You will need
to use Maildir. Dovecot also uses PAM, which is convenient. You
can use the same certificates as earlier for TLS.