%module morpheus
%{
#include "morpheus.hpp"
#include <cstdint>
%};

#ifdef SWIGJAVA
%rename("%(lowercamelcase)s", %$isfunction) "";
#endif

%nodefault;

/*

#include <cstdint>

typedef struct {

    %extend {

        static CPtrResult<Bip39> lang(const char *lang);

        Bip39();

        ~Bip39();

        CPtrResult<Bip39Phrase> entropy(const uint8_t *entropy);

        // CPtrResult<Bip39Phrase> short_entropy(const uint8_t *entropy);

        CPtrResult<Bip39Phrase> phrase(const char *phrase);

        // CPtrResult<Bip39Phrase> short_phrase(const char *phrase);

        CPtrResult<Bip39Phrase> generate();

        CPtrResult<RawSlice<char*>> list_words(const char *pref);

        CPtrResult<void> validate_phrase(const char *phrase);

    }

} Bip39;


typedef struct {

    %extend {

        ~Bip39Phrase();

        CPtrResult<Seed> password(password: string);

        %immutable phrase;
        CPtrResult<char> phrase;
    }

} Bip39Phrase;

typedef struct {

    %extend {
        ~Seed();
    }

} Seed;

*/

typedef struct {

    %extend {

        static CPtrResult<void> rewind(
            Vault *vault,
            const char *unlock_pwd,
            const char *network,
            int32_t account
        );

        static CPtrResult<HydraPlugin> get(
            Vault *vault,
            const char *network,
            int32_t account
        );

        ~HydraPlugin();

        // TODO remove this temporary
        CPtrResult<char> address(int32_t idx);

/*
        %immutable pub;
        CPtrResult<HydraPublic> pub;

        CPtrResult<HydraPrivate> priv(const char *unlock_pwd);
*/
    }

} HydraPlugin;

/*

typedef struct {

    %extend {

        ~HydraPublic();

        CPtrResult<Bip44PublicKey> key(int32_t idx);

        // CPtrResult<Bip44PublicKey> keyById(SecpKeyId *id);

    }

} HydraPublic;


typedef struct {

    %extend {

        ~Bip44PublicKey();

        CPtrResult<SecpPublicKey> publicKey();

        CPtrResult<SecpKeyId> keyId();

        %immutable
        CPtrResult<char> address;
    }

} Bip44PublicKey;


typedef struct {

    %extend {

        ~SecpPublicKey();
        
        CPtrResult<unsigned char> validateEcdsa(
            RawSlice<uint8_t> *data,
            SecpSignature *signature
        );

    }

} SecpPublicKey;


typedef struct {

    %extend {

        ~SecpKeyId();

    }

} SecpKeyId;


typedef struct {

    %extend {

        ~SecpSignature();

    }

} SecpSignature;


typedef struct {

    %extend {

        CPtrResult<Bip44Key> key(int32_t idx);

        CPtrResult<Bip44Key> keyByPublicKey(SecpPublicKey *pk);

        CPtrResult<Bip44Key> keyById(SecpKeyId *id);

        CPtrResult<SecpSignedBytes> sign(SecpKeyId *id, RawSlice<uint8_t> *data);

        %immutable pub;
        CPtrResult<HydraPublic> pub;

    }

} HydraPrivate;


typedef struct {

    %extend {

        CPtrResult<SecpPrivateKey> privateKey();

        // CPtrResult<Bip44PublicKey> neuter();

        // %immutable wif;
        // CPtrResult<char> wif;

    }

} Bip44Key;


typedef struct {

    %extend {

        // static CPtrResult<SecpPrivateKey> fromArkPassphrase(const char *phrase);

        CPtrResult<SecpPublicKey> publicKey();

        CPtrResult<SecpSignature> signEcdsa(RawSlice<uint8_t> *data);

    }

} SecpPrivateKey;

*/

typedef struct {

    %extend {

        static CPtrResult<void> rewind(
            Vault *vault,
            const char *unlock_pwd
        );

        static CPtrResult<MorpheusPlugin> get(
            Vault *vault
        );

        // TODO remove this temporary
        CPtrResult<char> persona(int32_t idx);

    }

} MorpheusPlugin;


typedef struct {
    %extend {

        static CPtrResult<Vault> create(
            const char *seed,
            const char *word25,
            const char *unlock_pwd
        );

        static CPtrResult<Vault> load(
            const char *json
        );

        ~Vault();

        CPtrResult<char> save();

        %immutable dirty;
        CPtrResult<unsigned char> dirty;

    }

} Vault;
