# CryptoTokenKit coverage audit

Audited against `MacOSX26.2.sdk/System/Library/Frameworks/CryptoTokenKit.framework/Headers`.

Legend:

- ✅ implemented
- 🟡 partial
- ⏭️ skipped

## TKError.h

| API | Status | Notes |
| --- | --- | --- |
| `TKErrorDomain` | 🟡 partial | `CryptoTokenKitError` preserves framework messages/codes, but the raw domain constant is not yet surfaced as a standalone Rust constant. |
| `TKErrorCode` | 🟡 partial | Numeric framework error codes flow through `CryptoTokenKitError::code()`, but the Apple enum is not re-exported verbatim. |

## TKTLVRecord.h

| API | Status | Notes |
| --- | --- | --- |
| `TKTLVTag` | ✅ implemented | Exposed through `TlvRecord::tag`. |
| `TKTLVRecord.tag / value / data` | ✅ implemented | Available on `TlvRecord`. |
| `TKTLVRecord.recordFromData:` | ⏭️ skipped | Calling the framework parser from Swift currently raises `NSInternalInconsistencyException` on macOS 26.2. |
| `TKTLVRecord.sequenceOfRecordsFromData:` | ⏭️ skipped | Same parser exception as `recordFromData:`. |
| `TKBERTLVRecord.dataForTag:` | ⏭️ skipped | Not surfaced separately; BER construction is available through `TlvRecord::ber`, but the standalone tag-encoding helper is deferred. |
| `TKBERTLVRecord.initWithTag:value:` | ✅ implemented | `TlvRecord::ber`. |
| `TKBERTLVRecord.initWithTag:records:` | ⏭️ skipped | Depends on constructing framework-owned child `TKTLVRecord` objects; not yet bridged. |
| `TKSimpleTLVRecord.initWithTag:value:` | ✅ implemented | `TlvRecord::simple`. |
| `TKCompactTLVRecord.initWithTag:value:` | ✅ implemented | `TlvRecord::compact`. |

## TKSmartCardATR.h

| API | Status | Notes |
| --- | --- | --- |
| `TKSmartCardProtocol` | ✅ implemented | Exposed as `SmartCardProtocol`. |
| `TKSmartCardATRInterfaceGroup.TA/TB/TC/protocol` | ✅ implemented | Reflected in `SmartCardAtrInterfaceGroup`. |
| `TKSmartCardATR.initWithBytes:` | ✅ implemented | `SmartCardAtr::parse`. |
| `TKSmartCardATR.initWithSource:` | ✅ implemented | `SmartCardAtr::parse_from_source`. |
| `TKSmartCardATR.bytes` | ✅ implemented | `SmartCardAtr::bytes`. |
| `TKSmartCardATR.protocols` | ✅ implemented | `SmartCardAtr::protocols`. |
| `TKSmartCardATR.interfaceGroupAtIndex:` | ✅ implemented | Flattened into `SmartCardAtr::interface_groups`. |
| `TKSmartCardATR.interfaceGroupForProtocol:` | ✅ implemented | Equivalent data available via `SmartCardAtr::interface_groups`. |
| `TKSmartCardATR.historicalBytes` | ✅ implemented | `SmartCardAtr::historical_bytes`. |
| `TKSmartCardATR.historicalRecords` | ✅ implemented | `SmartCardAtr::historical_records`. |

## TKSmartCard.h

| API | Status | Notes |
| --- | --- | --- |
| `TKSmartCardSlotManager.defaultManager` | ✅ implemented | `SmartCardSlotManager::default_manager`. |
| `TKSmartCardSlotManager.slotNames` | ✅ implemented | `SmartCardSlotManager::slot_names`. |
| `TKSmartCardSlotManager.getSlotWithName:reply:` | ✅ implemented | `SmartCardSlotManager::get_slot_with_name`. |
| `TKSmartCardSlotManager.slotNamed:` | ✅ implemented | `SmartCardSlotManager::slot_named`. |
| `TKSmartCardSlotManager.createNFCSlotWithMessage:completion:` | ⏭️ skipped | iOS-only NFC surface. |
| `TKSmartCardSlotManager.isNFCSupported` | ⏭️ skipped | iOS/macCatalyst-only NFC surface. |
| `TKSmartCardSlotState` | ✅ implemented | `SlotState`. |
| `TKSmartCardPINCharset` | ✅ implemented | `SmartCardPinCharset`. |
| `TKSmartCardPINEncoding` | ✅ implemented | `SmartCardPinEncoding`. |
| `TKSmartCardPINJustification` | ✅ implemented | `SmartCardPinJustification`. |
| `TKSmartCardPINCompletion` | ✅ implemented | `SmartCardPinCompletion`. |
| `TKSmartCardPINConfirmation` | ✅ implemented | `SmartCardPinConfirmation`. |
| `TKSmartCardPINFormat` | ✅ implemented | `SmartCardPinFormat`. |
| `TKSmartCardUserInteractionDelegate` | ⏭️ skipped | Reader UI callbacks require live secure-PIN interactions; not yet bridged. |
| `TKSmartCardUserInteraction.delegate / timeout / run / cancel` | ⏭️ skipped | Secure reader interaction objects are not yet exposed. |
| `TKSmartCardUserInteractionForPINOperation` | ⏭️ skipped | Depends on the unimplemented user-interaction bridge. |
| `TKSmartCardUserInteractionForSecurePINVerification` | ⏭️ skipped | Depends on the unimplemented user-interaction bridge. |
| `TKSmartCardUserInteractionForSecurePINChange` | ⏭️ skipped | Depends on the unimplemented user-interaction bridge. |
| `TKSmartCardSlot.state` | ✅ implemented | `SmartCardSlot::state`. |
| `TKSmartCardSlot.ATR` | ✅ implemented | `SmartCardSlot::atr`. |
| `TKSmartCardSlot.name` | ✅ implemented | `SmartCardSlot::name`. |
| `TKSmartCardSlot.maxInputLength` | ✅ implemented | `SmartCardSlot::max_input_length`. |
| `TKSmartCardSlot.maxOutputLength` | ✅ implemented | `SmartCardSlot::max_output_length`. |
| `TKSmartCardSlot.makeSmartCard` | ✅ implemented | `SmartCardSlot::make_smart_card`. |
| `TKSmartCard.slot` | 🟡 partial | `SmartCard::slot_name` is exposed, but a full `SmartCardSlot` handle accessor is deferred. |
| `TKSmartCard.valid` | ✅ implemented | `SmartCard::valid`. |
| `TKSmartCard.allowedProtocols` | ✅ implemented | `SmartCard::allowed_protocols` / `set_allowed_protocols`. |
| `TKSmartCard.currentProtocol` | ✅ implemented | `SmartCard::current_protocol`. |
| `TKSmartCard.sensitive` | ✅ implemented | `SmartCard::sensitive` / `set_sensitive`. |
| `TKSmartCard.context` | ✅ implemented | JSON string context via `SmartCard::context` / `set_context`. |
| `TKSmartCard.beginSessionWithReply:` | ✅ implemented | `SmartCard::begin_session`. |
| `TKSmartCard.transmitRequest:reply:` | ✅ implemented | `SmartCard::transmit_request`. |
| `TKSmartCard.endSession` | ✅ implemented | `SmartCard::end_session`. |
| `TKSmartCard.userInteractionForSecurePINVerification...` | ⏭️ skipped | Secure reader interaction bridge is deferred. |
| `TKSmartCard.userInteractionForSecurePINChange...` | ⏭️ skipped | Secure reader interaction bridge is deferred. |
| `TKSmartCard.cla` | ✅ implemented | `SmartCard::cla` / `set_cla`. |
| `TKSmartCard.useExtendedLength` | ✅ implemented | `SmartCard::use_extended_length` / `set_use_extended_length`. |
| `TKSmartCard.useCommandChaining` | ✅ implemented | `SmartCard::use_command_chaining` / `set_use_command_chaining`. |
| `TKSmartCard.sendIns:p1:p2:data:le:reply:` | ✅ implemented | `SmartCard::send_ins`. |
| `TKSmartCard.inSessionWithError:executeBlock:` | ✅ implemented | Rust-side `SmartCard::with_session` convenience. |
| `TKSmartCard.sendIns:p1:p2:data:le:sw:error:` | ✅ implemented | Rust-side `SmartCard::send_ins` exposes data and `status_word`. |

## TKToken.h

| API | Status | Notes |
| --- | --- | --- |
| `TKTokenObjectID` | ✅ implemented | `TokenObjectId`. |
| `_TKTokenObjectID` | ⏭️ skipped | Deprecated compatibility alias. |
| `TKTokenInstanceID` | ✅ implemented | Exposed through `TokenConfigurationSnapshot::instance_id` and token/session helpers. |
| `TKTokenDriverClassID` | ✅ implemented | Reflected in `TokenDriverConfigurationSnapshot::class_id`. |
| `TKTokenOperation` | ✅ implemented | `TokenOperation`. |
| `TKTokenOperationConstraint` | ✅ implemented | Stored as JSON-compatible values in `TokenKeychainItem::constraints`. |
| `TKTokenKeyAlgorithm` | ⏭️ skipped | Framework-generated objects have no public constructor; delegate-driven algorithm bridging is deferred. |
| `TKTokenKeyExchangeParameters` | ⏭️ skipped | Framework-generated objects have no public constructor; delegate-driven key-exchange bridging is deferred. |
| `TKTokenSession.initWithToken:` | ✅ implemented | `TokenSession::new`. |
| `TKTokenSession.token` | 🟡 partial | `TokenSession::token_instance_id` is exposed; full token-handle round-trip is deferred. |
| `TKTokenSession.delegate` | ⏭️ skipped | Extension-host callback bridge is deferred. |
| `TKTokenSessionDelegate` | ⏭️ skipped | Extension-host callback bridge is deferred. |
| `TKToken.initWithTokenDriver:instanceID:` | ✅ implemented | `Token::new`. |
| `TKToken.tokenDriver` | ⏭️ skipped | Full `TokenDriver` property round-trip is deferred. |
| `TKToken.delegate` | ⏭️ skipped | Extension-host callback bridge is deferred. |
| `TKToken.configuration` | ✅ implemented | `Token::configuration`. |
| `TKToken.keychainContents` | 🟡 partial | `Token::keychain_contents_items` reads framework-owned contents when present; base `TKToken` still reports `nil` outside extension host. |
| `TKTokenDelegate` | ⏭️ skipped | Extension-host callback bridge is deferred. |
| `TKTokenDriver.delegate` | ⏭️ skipped | Extension-host callback bridge is deferred. |
| `TKTokenDriverDelegate` | ⏭️ skipped | Extension-host callback bridge is deferred. |
| `TKTokenAuthOperation.finishWithError:` | ✅ implemented | `TokenAuthOperation::finish`. |
| `TKTokenPasswordAuthOperation.password` | ✅ implemented | `TokenPasswordAuthOperation::password` / `set_password`. |

## TKTokenConfiguration.h

| API | Status | Notes |
| --- | --- | --- |
| `TKTokenDriverConfiguration.driverConfigurations` | ✅ implemented | `TokenDriver::driver_configurations`. |
| `TKTokenDriverConfiguration.classID` | ✅ implemented | `TokenDriverConfigurationSnapshot::class_id`. |
| `TKTokenDriverConfiguration.tokenConfigurations` | ✅ implemented | `TokenDriverConfigurationSnapshot::token_configurations`. |
| `TKTokenDriverConfiguration.addTokenConfigurationForTokenInstanceID:` | ⏭️ skipped | Host-app-only mutation surface is deferred. |
| `TKTokenDriverConfiguration.removeTokenConfigurationForTokenInstanceID:` | ⏭️ skipped | Host-app-only mutation surface is deferred. |
| `TKTokenConfiguration.instanceID` | ✅ implemented | `TokenConfigurationSnapshot::instance_id`. |
| `TKTokenConfiguration.configurationData` | ✅ implemented | `Token::set_configuration_data` / `Token::configuration` (bridge-managed on base `TKToken`). |
| `TKTokenConfiguration.keychainItems` | ✅ implemented | `Token::set_keychain_items` / `Token::configuration`. |
| `TKTokenConfiguration.keyForObjectID:error:` | ✅ implemented | `Token::key_for_object_id`. |
| `TKTokenConfiguration.certificateForObjectID:error:` | ✅ implemented | `Token::certificate_for_object_id`. |

## TKTokenKeychainItem.h

| API | Status | Notes |
| --- | --- | --- |
| `TKTokenKeychainItem.initWithObjectID:` | ✅ implemented | `TokenKeychainItem::new`. |
| `TKTokenKeychainItem.objectID / label / constraints` | ✅ implemented | `TokenKeychainItem`. |
| `TKTokenKeychainCertificate.initWithCertificate:objectID:` | ✅ implemented | `TokenKeychainCertificate` round-trips through the Swift bridge. |
| `TKTokenKeychainCertificate.data` | ✅ implemented | `TokenKeychainCertificate::data`. |
| `TKTokenKeychainKey.initWithCertificate:objectID:` | ✅ implemented | `TokenKeychainKey` round-trips through the Swift bridge. |
| `TKTokenKeychainKey.keyType / applicationTag / keySizeInBits / publicKeyData / publicKeyHash / canDecrypt / canSign / canPerformKeyExchange / suitableForLogin` | ✅ implemented | `TokenKeychainKey` and `TokenKeyCapabilities`. |
| `TKTokenKeychainContents.fillWithItems:` | 🟡 partial | Equivalent functionality is available through `Token::set_keychain_items`, but `TKTokenKeychainContents` itself has no public constructor in the current bridge. |
| `TKTokenKeychainContents.items` | 🟡 partial | Exposed when `TKToken.keychainContents` is available through `Token::keychain_contents_items`. |
| `TKTokenKeychainContents.keyForObjectID:error:` | 🟡 partial | Equivalent lookup is exposed via `Token::key_for_object_id`. |
| `TKTokenKeychainContents.certificateForObjectID:error:` | 🟡 partial | Equivalent lookup is exposed via `Token::certificate_for_object_id`. |

## TKTokenWatcher.h

| API | Status | Notes |
| --- | --- | --- |
| `TKTokenWatcherTokenInfo.tokenID / slotName / driverName` | ✅ implemented | `TokenWatcherTokenInfo`. |
| `TKTokenWatcher.tokenIDs` | ✅ implemented | `TokenWatcher::token_ids`. |
| `TKTokenWatcher.init` | ✅ implemented | `TokenWatcher::new`. |
| `TKTokenWatcher.initWithInsertionHandler:` | ⏭️ skipped | Deprecated initializer; `setInsertionHandler` is implemented instead. |
| `TKTokenWatcher.setInsertionHandler:` | ✅ implemented | `TokenWatcher::set_insertion_handler`. |
| `TKTokenWatcher.addRemovalHandler:forTokenID:` | ✅ implemented | `TokenWatcher::add_removal_handler`. |
| `TKTokenWatcher.tokenInfoForTokenID:` | ✅ implemented | `TokenWatcher::token_info`. |

## TKSmartCardToken.h

| API | Status | Notes |
| --- | --- | --- |
| `TKTokenSmartCardPINAuthOperation.PINFormat / APDUTemplate / PINByteOffset / smartCard / PIN` | ✅ implemented | `TokenSmartCardPinAuthOperation`. |
| `TKSmartCardTokenSession.smartCard` | ✅ implemented | `SmartCardTokenSession::smart_card`. |
| `TKSmartCardTokenSession.getSmartCardWithError:` | ✅ implemented | `SmartCardTokenSession::get_smart_card` (macOS 26+). |
| `TKSmartCardToken.initWithSmartCard:AID:instanceID:tokenDriver:` | ✅ implemented | `SmartCardToken::new`. |
| `TKSmartCardToken.AID` | ✅ implemented | `SmartCardToken::aid`. |
| `TKSmartCardTokenDriver` | ✅ implemented | `SmartCardTokenDriver::new`. |
| `TKSmartCardTokenDriverDelegate` | ⏭️ skipped | Extension-host callback bridge is deferred. |

## TKSmartCardSlotNFCSession.h

| API | Status | Notes |
| --- | --- | --- |
| `TKSmartCardSlotNFCSession` | ⏭️ skipped | iOS/macCatalyst/visionOS-only NFC session surface. |

## TKSmartCardTokenRegistrationManager.h

| API | Status | Notes |
| --- | --- | --- |
| `TKSmartCardTokenRegistrationManager` | ⏭️ skipped | iOS/macCatalyst/visionOS-only registration manager surface. |
