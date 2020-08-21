var searchIndex = JSON.parse('{\
"advanca_core":{"doc":"","i":[[6,"Duration","advanca_core","Duration of the task. 0 means unlimited.",null,null],[6,"BlockNumber","","An index to a block.",null,null],[6,"Signature","","Alias to 512-bit hash when used in the context of a…",null,null],[6,"AccountId","","Some way of identifying an account on the chain. We…",null,null],[6,"AccountIndex","","The type for looking up accounts. We don\'t expect more…",null,null],[6,"Balance","","Balance of an account.",null,null],[6,"Index","","Index of a transaction in the chain.",null,null],[6,"Hash","","A hash of some data used by the chain.",null,null],[6,"DigestItem","","Digest item type.",null,null],[6,"Ciphertext","","The encrypted bytes",null,null],[3,"User","","The public information about a user",null,null],[12,"account_id","","User account on chain",0,null],[12,"public_keys","","User\'s public keys",0,null],[3,"Enclave","","The public information about an Enclave",null,null],[12,"account_id","","Enclave account on chain",1,null],[12,"public_keys","","Enclave\'s public keys",1,null],[12,"attestation","","Enclave attestation information which certifies all the…",1,null],[3,"Worker","","The public information about a worker and its enclave",null,null],[12,"account_id","","Worker account on chain",2,null],[12,"enclave","","Enclave that the worker is running",2,null],[4,"TaskStatus","","The status of a task managed on chain",null,null],[13,"Unscheduled","","The task is already submitted but not taken by any workers",3,null],[13,"Scheduled","","The task is taken by a worker",3,null],[13,"Done","","The task is completed or aborted",3,null],[4,"Privacy","","The possible privacy levels",null,null],[13,"None","","Public",4,null],[13,"Encryption","","Only encryption used",4,null],[13,"SqrtOram","","Encryption and Square-Root ORAM",4,null],[13,"PathOram","","Encryption and Path ORAM",4,null],[13,"OblivP2p","","Encryption and Oblivous P2P",4,null],[3,"TaskSpec","","The task specification. //TODO: more details",null,null],[12,"pod_spec","","The specification of the pod, which is the same concept in…",5,null],[12,"volume_spec","","The specification of the storage volume",5,null],[12,"privacy","","The privacy level",5,null],[3,"PublicKeys","","",null,null],[12,"secp256r1_public_key","","secp256r1 public key",6,null],[12,"sr25519_public_key","","sr25519 public key",6,null],[3,"Task","","Task information",null,null],[12,"task_id","","A unversial task ID that is unique for every submission by…",7,null],[12,"status","","The current task status",7,null],[12,"owner","","The user who submitted the task",7,null],[12,"signed_owner_task_secp256r1_pubkey","","The user\'s signed task key",7,null],[12,"signed_owner_task_sr25519_pubkey","","The user\'s signed task key",7,null],[12,"lease","","The owner-determined task duration. Default to 0 for…",7,null],[12,"task_spec","","The detailed specification of a task",7,null],[12,"worker","","The worker who accepted the task. It may be none at the…",7,null],[12,"signed_enclave_task_secp256r1_pubkey","","The worker\'s ephemeral key for the particular task signed…",7,null],[12,"signed_enclave_task_sr25519_pubkey","","The worker\'s ephemeral key for the particular task signed…",7,null],[12,"worker_url","","The worker\'s service url saved in ciphertext encrypted by…",7,null],[12,"worker_heartbeat_evidence","","Worker\'s heartbeat evidence",7,null],[3,"Module","","The module declaration.",null,null],[4,"Error","","",null,null],[13,"AlreadyRegistered","","",8,null],[13,"NotFound","","",8,null],[4,"Call","","Dispatchable calls.",null,null],[13,"register_worker","","",9,null],[13,"deregister_worker","","",9,null],[13,"register_user","","",9,null],[13,"deregister_user","","",9,null],[13,"submit_task","","",9,null],[13,"submit_task_evidence","","",9,null],[13,"update_task","","Updates a task",9,null],[13,"accept_task","","Worker accepts a task",9,null],[13,"abort_task","","",9,null],[13,"complete_task","","",9,null],[13,"progress_task","","",9,null],[4,"RawEvent","","Events for this module.",null,null],[13,"UserAdded","","",10,null],[13,"UserRemoved","","",10,null],[13,"TaskSubmitted","","",10,null],[13,"TaskUpdated","","",10,null],[13,"TaskAccepted","","",10,null],[13,"TaskCompleted","","",10,null],[13,"TaskAborted","","",10,null],[13,"WorkerAdded","","",10,null],[13,"WorkerRemoved","","",10,null],[5,"sr25519_verify_msg","","",null,[[["sr25519signedmsg",3],["sr25519publickey",3]]]],[5,"sr25519_verify_signature","","",null,[[["sr25519signature",3],["sr25519publickey",3]]]],[6,"BalanceOf","","",null,null],[6,"TaskId","","",null,null],[6,"Index","","",null,null],[6,"Event","","[`RawEvent`] specialized for the configuration [`Trait`]",null,null],[8,"Trait","","The module\'s configuration trait.",null,null],[16,"Event","","The overarching event type.",11,null],[16,"Currency","","The currency to be handled in this module",11,null],[11,"get_user","","Registered users",12,[[["encodelike",8]],["user",3]]],[11,"get_worker","","Registered workers",12,[[["encodelike",8]],["worker",3]]],[11,"get_task","","Saved tasks",12,[[["encodelike",8],["taskid",6]],[["taskid",6],["task",3],["taskspec",3],["duration",6],["taskstatus",4],["ciphertext",6]]]],[11,"unscheduled_tasks","","Unscheduled tasks",12,[[],[["vec",3],["taskid",6]]]],[11,"register_worker","","NOTE: Calling this function will bypass origin filters.",12,[[["enclave",3],["balanceof",6]],["dispatchresult",6]]],[11,"deregister_worker","","NOTE: Calling this function will bypass origin filters.",12,[[],["dispatchresult",6]]],[11,"register_user","","NOTE: Calling this function will bypass origin filters.",12,[[["publickeys",3],["balanceof",6]],["dispatchresult",6]]],[11,"deregister_user","","NOTE: Calling this function will bypass origin filters.",12,[[],["dispatchresult",6]]],[11,"submit_task","","NOTE: Calling this function will bypass origin filters.",12,[[["taskspec",3],["vec",3],["duration",6],["privacy",4]],["dispatchresult",6]]],[11,"submit_task_evidence","","NOTE: Calling this function will bypass origin filters.",12,[[["taskid",6],["vec",3],["vec",3]],["dispatchresult",6]]],[11,"update_task","","Updates a task",12,[[["taskid",6],["taskspec",3],["privacy",4]],["dispatchresult",6]]],[11,"accept_task","","Worker accepts a task",12,[[["taskid",6],["vec",3],["ciphertext",6]],["dispatchresult",6]]],[11,"abort_task","","NOTE: Calling this function will bypass origin filters.",12,[[["taskid",6]],["dispatchresult",6]]],[11,"complete_task","","NOTE: Calling this function will bypass origin filters.",12,[[["taskid",6]],["dispatchresult",6]]],[11,"progress_task","","NOTE: Calling this function will bypass origin filters.",12,[[],["dispatchresult",6]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"vzip","","",0,[[]]],[11,"decode","","",0,[[],[["error",3],["result",4]]]],[11,"size_hint","","",0,[[]]],[11,"using_encoded","","",0,[[]]],[11,"encode","","",0,[[],["vec",3]]],[11,"encode_to","","",0,[[]]],[11,"blake2_128","","",0,[[]]],[11,"blake2_256","","",0,[[]]],[11,"blake2_128_concat","","",0,[[],["vec",3]]],[11,"twox_128","","",0,[[]]],[11,"twox_256","","",0,[[]]],[11,"twox_64_concat","","",0,[[],["vec",3]]],[11,"identity","","",0,[[],["vec",3]]],[11,"from_ref","","",0,[[]]],[11,"into_ref","","",0,[[]]],[11,"from_mut","","",0,[[]]],[11,"into_mut","","",0,[[]]],[11,"to_keyed_vec","","",0,[[],["vec",3]]],[11,"decode_all","","",0,[[],[["error",3],["result",4]]]],[11,"decode_all_with_depth_limit","","",0,[[],[["error",3],["result",4]]]],[11,"decode_with_depth_limit","","",0,[[],[["error",3],["result",4]]]],[11,"is_clear","","",0,[[]]],[11,"clear","","",0,[[]]],[11,"unchecked_into","","",0,[[]]],[11,"from_ref","","Get a reference to the inner from the outer.",0,[[]]],[11,"from_mut","","Get a mutable reference to the inner from the outer.",0,[[]]],[11,"unique_saturated_into","","",0,[[]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"vzip","","",1,[[]]],[11,"decode","","",1,[[],[["error",3],["result",4]]]],[11,"size_hint","","",1,[[]]],[11,"using_encoded","","",1,[[]]],[11,"encode","","",1,[[],["vec",3]]],[11,"encode_to","","",1,[[]]],[11,"blake2_128","","",1,[[]]],[11,"blake2_256","","",1,[[]]],[11,"blake2_128_concat","","",1,[[],["vec",3]]],[11,"twox_128","","",1,[[]]],[11,"twox_256","","",1,[[]]],[11,"twox_64_concat","","",1,[[],["vec",3]]],[11,"identity","","",1,[[],["vec",3]]],[11,"from_ref","","",1,[[]]],[11,"into_ref","","",1,[[]]],[11,"from_mut","","",1,[[]]],[11,"into_mut","","",1,[[]]],[11,"to_keyed_vec","","",1,[[],["vec",3]]],[11,"decode_all","","",1,[[],[["error",3],["result",4]]]],[11,"decode_all_with_depth_limit","","",1,[[],[["error",3],["result",4]]]],[11,"decode_with_depth_limit","","",1,[[],[["error",3],["result",4]]]],[11,"is_clear","","",1,[[]]],[11,"clear","","",1,[[]]],[11,"unchecked_into","","",1,[[]]],[11,"from_ref","","Get a reference to the inner from the outer.",1,[[]]],[11,"from_mut","","Get a mutable reference to the inner from the outer.",1,[[]]],[11,"unique_saturated_into","","",1,[[]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_owned","","",2,[[]]],[11,"clone_into","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"vzip","","",2,[[]]],[11,"decode","","",2,[[],[["error",3],["result",4]]]],[11,"size_hint","","",2,[[]]],[11,"using_encoded","","",2,[[]]],[11,"encode","","",2,[[],["vec",3]]],[11,"encode_to","","",2,[[]]],[11,"blake2_128","","",2,[[]]],[11,"blake2_256","","",2,[[]]],[11,"blake2_128_concat","","",2,[[],["vec",3]]],[11,"twox_128","","",2,[[]]],[11,"twox_256","","",2,[[]]],[11,"twox_64_concat","","",2,[[],["vec",3]]],[11,"identity","","",2,[[],["vec",3]]],[11,"from_ref","","",2,[[]]],[11,"into_ref","","",2,[[]]],[11,"from_mut","","",2,[[]]],[11,"into_mut","","",2,[[]]],[11,"to_keyed_vec","","",2,[[],["vec",3]]],[11,"decode_all","","",2,[[],[["error",3],["result",4]]]],[11,"decode_all_with_depth_limit","","",2,[[],[["error",3],["result",4]]]],[11,"decode_with_depth_limit","","",2,[[],[["error",3],["result",4]]]],[11,"is_clear","","",2,[[]]],[11,"clear","","",2,[[]]],[11,"unchecked_into","","",2,[[]]],[11,"from_ref","","Get a reference to the inner from the outer.",2,[[]]],[11,"from_mut","","Get a mutable reference to the inner from the outer.",2,[[]]],[11,"unique_saturated_into","","",2,[[]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"to_owned","","",3,[[]]],[11,"clone_into","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"vzip","","",3,[[]]],[11,"decode","","",3,[[],[["error",3],["result",4]]]],[11,"size_hint","","",3,[[]]],[11,"using_encoded","","",3,[[]]],[11,"encode","","",3,[[],["vec",3]]],[11,"encode_to","","",3,[[]]],[11,"blake2_128","","",3,[[]]],[11,"blake2_256","","",3,[[]]],[11,"blake2_128_concat","","",3,[[],["vec",3]]],[11,"twox_128","","",3,[[]]],[11,"twox_256","","",3,[[]]],[11,"twox_64_concat","","",3,[[],["vec",3]]],[11,"identity","","",3,[[],["vec",3]]],[11,"from_ref","","",3,[[]]],[11,"into_ref","","",3,[[]]],[11,"from_mut","","",3,[[]]],[11,"into_mut","","",3,[[]]],[11,"to_keyed_vec","","",3,[[],["vec",3]]],[11,"decode_all","","",3,[[],[["error",3],["result",4]]]],[11,"decode_all_with_depth_limit","","",3,[[],[["error",3],["result",4]]]],[11,"decode_with_depth_limit","","",3,[[],[["error",3],["result",4]]]],[11,"is_clear","","",3,[[]]],[11,"clear","","",3,[[]]],[11,"unchecked_into","","",3,[[]]],[11,"from_ref","","Get a reference to the inner from the outer.",3,[[]]],[11,"from_mut","","Get a mutable reference to the inner from the outer.",3,[[]]],[11,"unique_saturated_into","","",3,[[]]],[11,"from","","",4,[[]]],[11,"into","","",4,[[]]],[11,"to_owned","","",4,[[]]],[11,"clone_into","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"vzip","","",4,[[]]],[11,"decode","","",4,[[],[["error",3],["result",4]]]],[11,"size_hint","","",4,[[]]],[11,"using_encoded","","",4,[[]]],[11,"encode","","",4,[[],["vec",3]]],[11,"encode_to","","",4,[[]]],[11,"blake2_128","","",4,[[]]],[11,"blake2_256","","",4,[[]]],[11,"blake2_128_concat","","",4,[[],["vec",3]]],[11,"twox_128","","",4,[[]]],[11,"twox_256","","",4,[[]]],[11,"twox_64_concat","","",4,[[],["vec",3]]],[11,"identity","","",4,[[],["vec",3]]],[11,"from_ref","","",4,[[]]],[11,"into_ref","","",4,[[]]],[11,"from_mut","","",4,[[]]],[11,"into_mut","","",4,[[]]],[11,"to_keyed_vec","","",4,[[],["vec",3]]],[11,"decode_all","","",4,[[],[["error",3],["result",4]]]],[11,"decode_all_with_depth_limit","","",4,[[],[["error",3],["result",4]]]],[11,"decode_with_depth_limit","","",4,[[],[["error",3],["result",4]]]],[11,"is_clear","","",4,[[]]],[11,"clear","","",4,[[]]],[11,"unchecked_into","","",4,[[]]],[11,"from_ref","","Get a reference to the inner from the outer.",4,[[]]],[11,"from_mut","","Get a mutable reference to the inner from the outer.",4,[[]]],[11,"unique_saturated_into","","",4,[[]]],[11,"from","","",5,[[]]],[11,"into","","",5,[[]]],[11,"to_owned","","",5,[[]]],[11,"clone_into","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"vzip","","",5,[[]]],[11,"decode","","",5,[[],[["error",3],["result",4]]]],[11,"size_hint","","",5,[[]]],[11,"using_encoded","","",5,[[]]],[11,"encode","","",5,[[],["vec",3]]],[11,"encode_to","","",5,[[]]],[11,"blake2_128","","",5,[[]]],[11,"blake2_256","","",5,[[]]],[11,"blake2_128_concat","","",5,[[],["vec",3]]],[11,"twox_128","","",5,[[]]],[11,"twox_256","","",5,[[]]],[11,"twox_64_concat","","",5,[[],["vec",3]]],[11,"identity","","",5,[[],["vec",3]]],[11,"from_ref","","",5,[[]]],[11,"into_ref","","",5,[[]]],[11,"from_mut","","",5,[[]]],[11,"into_mut","","",5,[[]]],[11,"to_keyed_vec","","",5,[[],["vec",3]]],[11,"decode_all","","",5,[[],[["error",3],["result",4]]]],[11,"decode_all_with_depth_limit","","",5,[[],[["error",3],["result",4]]]],[11,"decode_with_depth_limit","","",5,[[],[["error",3],["result",4]]]],[11,"is_clear","","",5,[[]]],[11,"clear","","",5,[[]]],[11,"unchecked_into","","",5,[[]]],[11,"from_ref","","Get a reference to the inner from the outer.",5,[[]]],[11,"from_mut","","Get a mutable reference to the inner from the outer.",5,[[]]],[11,"unique_saturated_into","","",5,[[]]],[11,"from","","",6,[[]]],[11,"into","","",6,[[]]],[11,"to_owned","","",6,[[]]],[11,"clone_into","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"vzip","","",6,[[]]],[11,"decode","","",6,[[],[["error",3],["result",4]]]],[11,"size_hint","","",6,[[]]],[11,"using_encoded","","",6,[[]]],[11,"encode","","",6,[[],["vec",3]]],[11,"encode_to","","",6,[[]]],[11,"blake2_128","","",6,[[]]],[11,"blake2_256","","",6,[[]]],[11,"blake2_128_concat","","",6,[[],["vec",3]]],[11,"twox_128","","",6,[[]]],[11,"twox_256","","",6,[[]]],[11,"twox_64_concat","","",6,[[],["vec",3]]],[11,"identity","","",6,[[],["vec",3]]],[11,"from_ref","","",6,[[]]],[11,"into_ref","","",6,[[]]],[11,"from_mut","","",6,[[]]],[11,"into_mut","","",6,[[]]],[11,"to_keyed_vec","","",6,[[],["vec",3]]],[11,"decode_all","","",6,[[],[["error",3],["result",4]]]],[11,"decode_all_with_depth_limit","","",6,[[],[["error",3],["result",4]]]],[11,"decode_with_depth_limit","","",6,[[],[["error",3],["result",4]]]],[11,"is_clear","","",6,[[]]],[11,"clear","","",6,[[]]],[11,"unchecked_into","","",6,[[]]],[11,"from_ref","","Get a reference to the inner from the outer.",6,[[]]],[11,"from_mut","","Get a mutable reference to the inner from the outer.",6,[[]]],[11,"unique_saturated_into","","",6,[[]]],[11,"from","","",7,[[]]],[11,"into","","",7,[[]]],[11,"to_owned","","",7,[[]]],[11,"clone_into","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"vzip","","",7,[[]]],[11,"decode","","",7,[[],[["error",3],["result",4]]]],[11,"size_hint","","",7,[[]]],[11,"using_encoded","","",7,[[]]],[11,"encode","","",7,[[],["vec",3]]],[11,"encode_to","","",7,[[]]],[11,"blake2_128","","",7,[[]]],[11,"blake2_256","","",7,[[]]],[11,"blake2_128_concat","","",7,[[],["vec",3]]],[11,"twox_128","","",7,[[]]],[11,"twox_256","","",7,[[]]],[11,"twox_64_concat","","",7,[[],["vec",3]]],[11,"identity","","",7,[[],["vec",3]]],[11,"from_ref","","",7,[[]]],[11,"into_ref","","",7,[[]]],[11,"from_mut","","",7,[[]]],[11,"into_mut","","",7,[[]]],[11,"to_keyed_vec","","",7,[[],["vec",3]]],[11,"decode_all","","",7,[[],[["error",3],["result",4]]]],[11,"decode_all_with_depth_limit","","",7,[[],[["error",3],["result",4]]]],[11,"decode_with_depth_limit","","",7,[[],[["error",3],["result",4]]]],[11,"is_clear","","",7,[[]]],[11,"clear","","",7,[[]]],[11,"unchecked_into","","",7,[[]]],[11,"from_ref","","Get a reference to the inner from the outer.",7,[[]]],[11,"from_mut","","Get a mutable reference to the inner from the outer.",7,[[]]],[11,"unique_saturated_into","","",7,[[]]],[11,"from","","",12,[[]]],[11,"into","","",12,[[]]],[11,"to_owned","","",12,[[]]],[11,"clone_into","","",12,[[]]],[11,"try_from","","",12,[[],["result",4]]],[11,"try_into","","",12,[[],["result",4]]],[11,"borrow","","",12,[[]]],[11,"borrow_mut","","",12,[[]]],[11,"type_id","","",12,[[],["typeid",3]]],[11,"vzip","","",12,[[]]],[11,"from_ref","","",12,[[]]],[11,"into_ref","","",12,[[]]],[11,"from_mut","","",12,[[]]],[11,"into_mut","","",12,[[]]],[11,"unchecked_into","","",12,[[]]],[11,"from_ref","","Get a reference to the inner from the outer.",12,[[]]],[11,"from_mut","","Get a mutable reference to the inner from the outer.",12,[[]]],[11,"unique_saturated_into","","",12,[[]]],[11,"from","","",8,[[]]],[11,"into","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"vzip","","",8,[[]]],[11,"with_weight","","",8,[[],[["dispatcherrorwithpostinfo",3],["postdispatchinfo",3]]]],[11,"from_ref","","",8,[[]]],[11,"into_ref","","",8,[[]]],[11,"from_mut","","",8,[[]]],[11,"into_mut","","",8,[[]]],[11,"unchecked_into","","",8,[[]]],[11,"from_ref","","Get a reference to the inner from the outer.",8,[[]]],[11,"from_mut","","Get a mutable reference to the inner from the outer.",8,[[]]],[11,"unique_saturated_into","","",8,[[]]],[11,"from","","",9,[[]]],[11,"into","","",9,[[]]],[11,"to_owned","","",9,[[]]],[11,"clone_into","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"try_into","","",9,[[],["result",4]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"vzip","","",9,[[]]],[11,"decode","","",9,[[],[["error",3],["result",4]]]],[11,"size_hint","","",9,[[]]],[11,"using_encoded","","",9,[[]]],[11,"encode","","",9,[[],["vec",3]]],[11,"encode_to","","",9,[[]]],[11,"blake2_128","","",9,[[]]],[11,"blake2_256","","",9,[[]]],[11,"blake2_128_concat","","",9,[[],["vec",3]]],[11,"twox_128","","",9,[[]]],[11,"twox_256","","",9,[[]]],[11,"twox_64_concat","","",9,[[],["vec",3]]],[11,"identity","","",9,[[],["vec",3]]],[11,"from_ref","","",9,[[]]],[11,"into_ref","","",9,[[]]],[11,"from_mut","","",9,[[]]],[11,"into_mut","","",9,[[]]],[11,"to_keyed_vec","","",9,[[],["vec",3]]],[11,"decode_all","","",9,[[],[["error",3],["result",4]]]],[11,"decode_all_with_depth_limit","","",9,[[],[["error",3],["result",4]]]],[11,"decode_with_depth_limit","","",9,[[],[["error",3],["result",4]]]],[11,"unchecked_into","","",9,[[]]],[11,"from_ref","","Get a reference to the inner from the outer.",9,[[]]],[11,"from_mut","","Get a mutable reference to the inner from the outer.",9,[[]]],[11,"unique_saturated_into","","",9,[[]]],[11,"from","","",10,[[]]],[11,"into","","",10,[[]]],[11,"to_owned","","",10,[[]]],[11,"clone_into","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"try_into","","",10,[[],["result",4]]],[11,"borrow","","",10,[[]]],[11,"borrow_mut","","",10,[[]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"vzip","","",10,[[]]],[11,"decode","","",10,[[],[["error",3],["result",4]]]],[11,"size_hint","","",10,[[]]],[11,"using_encoded","","",10,[[]]],[11,"encode","","",10,[[],["vec",3]]],[11,"encode_to","","",10,[[]]],[11,"blake2_128","","",10,[[]]],[11,"blake2_256","","",10,[[]]],[11,"blake2_128_concat","","",10,[[],["vec",3]]],[11,"twox_128","","",10,[[]]],[11,"twox_256","","",10,[[]]],[11,"twox_64_concat","","",10,[[],["vec",3]]],[11,"identity","","",10,[[],["vec",3]]],[11,"from_ref","","",10,[[]]],[11,"into_ref","","",10,[[]]],[11,"from_mut","","",10,[[]]],[11,"into_mut","","",10,[[]]],[11,"to_keyed_vec","","",10,[[],["vec",3]]],[11,"decode_all","","",10,[[],[["error",3],["result",4]]]],[11,"decode_all_with_depth_limit","","",10,[[],[["error",3],["result",4]]]],[11,"decode_with_depth_limit","","",10,[[],[["error",3],["result",4]]]],[11,"unchecked_into","","",10,[[]]],[11,"from_ref","","Get a reference to the inner from the outer.",10,[[]]],[11,"from_mut","","Get a mutable reference to the inner from the outer.",10,[[]]],[11,"unique_saturated_into","","",10,[[]]],[11,"clone","","",0,[[],["user",3]]],[11,"clone","","",3,[[],["taskstatus",4]]],[11,"clone","","",6,[[],["publickeys",3]]],[11,"clone","","",4,[[],["privacy",4]]],[11,"clone","","",2,[[],["worker",3]]],[11,"clone","","",7,[[],["task",3]]],[11,"clone","","",5,[[],["taskspec",3]]],[11,"clone","","",1,[[],["enclave",3]]],[11,"eq","","",7,[[["task",3]]]],[11,"ne","","",7,[[["task",3]]]],[11,"eq","","",2,[[["worker",3]]]],[11,"ne","","",2,[[["worker",3]]]],[11,"eq","","",1,[[["enclave",3]]]],[11,"ne","","",1,[[["enclave",3]]]],[11,"eq","","",6,[[["publickeys",3]]]],[11,"ne","","",6,[[["publickeys",3]]]],[11,"eq","","",5,[[["taskspec",3]]]],[11,"ne","","",5,[[["taskspec",3]]]],[11,"eq","","",3,[[["taskstatus",4]]]],[11,"eq","","",4,[[["privacy",4]]]],[11,"eq","","",0,[[["user",3]]]],[11,"ne","","",0,[[["user",3]]]],[11,"default","","",6,[[],["publickeys",3]]],[11,"default","","",5,[[],["taskspec",3]]],[11,"default","","",2,[[],["worker",3]]],[11,"default","","Return `TaskStatus::Unscheduled`",3,[[],["taskstatus",4]]],[11,"default","","Return `Privacy::None`",4,[[],["privacy",4]]],[11,"default","","",7,[[],["task",3]]],[11,"default","","",1,[[],["enclave",3]]],[11,"default","","",0,[[],["user",3]]],[11,"decode","","",1,[[],[["result",4],["enclave",3],["error",3]]]],[11,"decode","","",7,[[],[["task",3],["result",4],["error",3]]]],[11,"decode","","",3,[[],[["taskstatus",4],["result",4],["error",3]]]],[11,"decode","","",4,[[],[["privacy",4],["error",3],["result",4]]]],[11,"decode","","",5,[[],[["taskspec",3],["error",3],["result",4]]]],[11,"decode","","",2,[[],[["worker",3],["result",4],["error",3]]]],[11,"decode","","",0,[[],[["user",3],["error",3],["result",4]]]],[11,"decode","","",6,[[],[["error",3],["result",4],["publickeys",3]]]],[11,"fmt","","",4,[[["formatter",3]],[["result",4],["error",3]]]],[11,"fmt","","",7,[[["formatter",3]],[["result",4],["error",3]]]],[11,"fmt","","",0,[[["formatter",3]],[["result",4],["error",3]]]],[11,"fmt","","",6,[[["formatter",3]],[["result",4],["error",3]]]],[11,"fmt","","",5,[[["formatter",3]],[["result",4],["error",3]]]],[11,"fmt","","",2,[[["formatter",3]],[["result",4],["error",3]]]],[11,"fmt","","",3,[[["formatter",3]],[["result",4],["error",3]]]],[11,"fmt","","",1,[[["formatter",3]],[["result",4],["error",3]]]],[11,"encode_to","","",4,[[]]],[11,"encode_to","","",6,[[]]],[11,"encode_to","","",3,[[]]],[11,"encode_to","","",1,[[]]],[11,"encode_to","","",2,[[]]],[11,"encode_to","","",7,[[]]],[11,"encode_to","","",5,[[]]],[11,"encode_to","","",0,[[]]],[11,"clone","","",12,[[],["module",3]]],[11,"clone","","",9,[[]]],[11,"clone","","",10,[[],["rawevent",4]]],[11,"eq","","",12,[[["module",3]]]],[11,"ne","","",12,[[["module",3]]]],[11,"eq","","",9,[[]]],[11,"eq","","",10,[[["rawevent",4]]]],[11,"ne","","",10,[[["rawevent",4]]]],[11,"fmt","","",8,[[["formatter",3]],["result",6]]],[11,"fmt","","",12,[[["formatter",3]],["result",6]]],[11,"fmt","","",9,[[["formatter",3]],[["result",4],["error",3]]]],[11,"fmt","","",10,[[["formatter",3]],["result",6]]],[11,"decode","","",9,[[],[["error",3],["result",4]]]],[11,"decode","","",10,[[],[["error",3],["result",4]]]],[11,"encode_to","","",9,[[]]],[11,"encode_to","","",10,[[]]],[11,"metadata","","",8,[[]]],[11,"metadata","","",12,[[]]],[11,"get_dispatch_info","","",9,[[],["dispatchinfo",3]]],[11,"get_call_name","","",9,[[]]],[11,"get_call_names","","",9,[[]]],[11,"dispatch_bypass_filter","","",9,[[],["dispatchresultwithpostinfo",6]]]],"p":[[3,"User"],[3,"Enclave"],[3,"Worker"],[4,"TaskStatus"],[4,"Privacy"],[3,"TaskSpec"],[3,"PublicKeys"],[3,"Task"],[4,"Error"],[4,"Call"],[4,"RawEvent"],[8,"Trait"],[3,"Module"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);