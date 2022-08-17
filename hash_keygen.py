import hashlib
#import random
import sys
import string


if len(sys.argv) != 2:
    print("hash_keygen <key>")
    sys.exit()

#key = ''.join([random.choice(string.ascii_letters) for i in range(0, 20)]).encode('utf-8')
key = b"this_is_the_key_1337"
key_hash = hashlib.sha256(key).hexdigest()

arg_hash = hashlib.sha256(sys.argv[1].encode('utf-8')).hexdigest()

if key_hash == arg_hash:
    print("Activated!")
else:
    print("Wrong key!")
