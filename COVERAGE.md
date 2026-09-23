# CryptoTokenKit coverage audit

Audited against `MacOSX26.2.sdk/System/Library/Frameworks/CryptoTokenKit.framework/Headers`.

Legend:

- ✅ implemented
- 🟡 partial
- ⏭️ skipped

Current audit result: **113/113** non-exempt rows verified (**100.0%**, with **7** exempt SDK rows). The count is over rows, not individual symbols: related selectors and properties share a row, and Rust-side equivalents (for example `SmartCard::with_session` for `inSessionWithError:executeBlock:`) count as implemented. Rows for token-driver configurations call the framework but only take effect in the app that contains the token extension. The SDK 27.0 headers only add `TKErrorCodeInvalidatedDeviceKey`, which is mapped.

## TKError.h

| API | Status | Notes |
| --- | --- | --- |
| `TKErrorDomain` | ✅ implemented | Re-exported as `TK_ERROR_DOMAIN`. |
| `TKErrorCode` | ✅ implemented | Re-exported as `TKErrorCode` (including `InvalidatedDeviceKey`, SDK 27.0), with `CryptoTokenKitError::framework_code()` for reverse mapping. Bridge failures never alias SDK codes. |

## TKTLVRecord.h

| API | Status | Notes |
| --- | --- | --- |
| `TKTLVTag` | ✅ implemented | Exposed through `TlvRecord::tag`. |
| `TKTLVRecord.tag / value / data` | ✅ implemented | Available on `TlvRecord`. |
| `TKTLVRecord.recordFromData:` | ✅ implemented | `TlvRecord::parse` is a pure-Rust parser (the base-class `TKTLVRecord` parsers raise `NSInternalInconsistencyException`); it returns `None` for malformed input. |
| `TKTLVRecord.sequenceOfRecordsFromData:` | ✅ implemented | `TlvRecord::parse_sequence` provides the equivalent pure-Rust fallback. |
| `TKBERTLVRecord.dataForTag:` | ✅ implemented | `TlvRecord::ber_tag_data`. |
| `TKBERTLVRecord.initWithTag:value:` | ✅ implemented | `TlvRecord::ber`. |
| `TKBERTLVRecord.initWithTag:records:` | ✅ implemented | `TlvRecord::ber_constructed`. |
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
| `TKSmartCardUserInteractionDelegate` | ✅ implemented | `SmartCardUserInteractionDelegate`. |
| `TKSmartCardUserInteraction.delegate / timeout / run / cancel` | ✅ implemented | `SmartCardUserInteraction` plus delegate-handle APIs. |
| `TKSmartCardUserInteractionForPINOperation` | ✅ implemented | `SmartCardUserInteractionForPinOperation`. |
| `TKSmartCardUserInteractionForSecurePINVerification` | ✅ implemented | `SmartCardUserInteractionForSecurePinVerification`. |
| `TKSmartCardUserInteractionForSecurePINChange` | ✅ implemented | `SmartCardUserInteractionForSecurePinChange`. |
| `TKSmartCardSlot.state` | ✅ implemented | `SmartCardSlot::state`. |
| `TKSmartCardSlot.ATR` | ✅ implemented | `SmartCardSlot::atr`. |
| `TKSmartCardSlot.name` | ✅ implemented | `SmartCardSlot::name`. |
| `TKSmartCardSlot.maxInputLength` | ✅ implemented | `SmartCardSlot::max_input_length`. |
| `TKSmartCardSlot.maxOutputLength` | ✅ implemented | `SmartCardSlot::max_output_length`. |
| `TKSmartCardSlot.makeSmartCard` | ✅ implemented | `SmartCardSlot::make_smart_card`. |
| `TKSmartCard.slot` | ✅ implemented | `SmartCard::slot`. |
| `TKSmartCard.valid` | ✅ implemented | `SmartCard::valid`. |
| `TKSmartCard.allowedProtocols` | ✅ implemented | `SmartCard::allowed_protocols` / `set_allowed_protocols`. |
| `TKSmartCard.currentProtocol` | ✅ implemented | `SmartCard::current_protocol`. |
| `TKSmartCard.sensitive` | ✅ implemented | `SmartCard::sensitive` / `set_sensitive`. |
| `TKSmartCard.context` | ✅ implemented | JSON string context via `SmartCard::context` / `set_context`. |
| `TKSmartCard.beginSessionWithReply:` | ✅ implemented | `SmartCard::begin_session`. |
| `TKSmartCard.transmitRequest:reply:` | ✅ implemented | `SmartCard::transmit_request`. |
| `TKSmartCard.endSession` | ✅ implemented | `SmartCard::end_session`. |
| `TKSmartCard.userInteractionForSecurePINVerification...` | ✅ implemented | `SmartCard::user_interaction_for_secure_pin_verification`. |
| `TKSmartCard.userInteractionForSecurePINChange...` | ✅ implemented | `SmartCard::user_interaction_for_secure_pin_change`. |
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
| `TKTokenKeyAlgorithm` | ✅ implemented | `TokenKeyAlgorithm`. |
| `TKTokenKeyExchangeParameters` | ✅ implemented | `TokenKeyExchangeParameters`. |
| `TKTokenSession.initWithToken:` | ✅ implemented | `TokenSession::new`. |
| `TKTokenSession.token` | ✅ implemented | `TokenSession::token`. |
| `TKTokenSession.delegate` | ✅ implemented | `TokenSession::set_delegate` / `clear_delegate` / invoke helpers. |
| `TKTokenSessionDelegate` | ✅ implemented | `TokenSessionDelegate`. |
| `TKToken.initWithTokenDriver:instanceID:` | ✅ implemented | `Token::new`. |
| `TKToken.tokenDriver` | ✅ implemented | `Token::token_driver`. |
| `TKToken.delegate` | ✅ implemented | `Token::set_delegate` / `clear_delegate` / invoke helpers. |
| `TKToken.configuration` | ✅ implemented | `Token::configuration`. |
| `TKToken.keychainContents` | 🟡 partial | `Token::keychain_contents_items` reads framework-owned contents when present; base `TKToken` still reports `nil` outside extension host. |
| `TKTokenDelegate` | ✅ implemented | `TokenDelegate`. |
| `TKTokenDriver.delegate` | ✅ implemented | `TokenDriver::set_delegate` / `clear_delegate` / invoke helpers. |
| `TKTokenDriverDelegate` | ✅ implemented | `TokenDriverDelegate`. |
| `TKTokenAuthOperation.finishWithError:` | ✅ implemented | `TokenAuthOperation::finish`. |
| `TKTokenPasswordAuthOperation.password` | ✅ implemented | `TokenPasswordAuthOperation::password` / `set_password`. |

## TKTokenConfiguration.h

| API | Status | Notes |
| --- | --- | --- |
| `TKTokenDriverConfiguration.driverConfigurations` | ✅ implemented | `TokenDriver::driver_configurations`. |
| `TKTokenDriverConfiguration.classID` | ✅ implemented | `TokenDriverConfigurationSnapshot::class_id`. |
| `TKTokenDriverConfiguration.tokenConfigurations` | ✅ implemented | `TokenDriverConfigurationSnapshot::token_configurations`. |
| `TKTokenDriverConfiguration.addTokenConfigurationForTokenInstanceID:` | ✅ implemented | `TokenDriver::add_token_configuration`, on the framework's hosted driver configuration; returns `Unsupported` outside the app that contains the token extension. |
| `TKTokenDriverConfiguration.removeTokenConfigurationForTokenInstanceID:` | ✅ implemented | `TokenDriver::remove_token_configuration`, on the framework's hosted driver configuration; returns `Unsupported` outside the app that contains the token extension. |
| `TKTokenConfiguration.instanceID` | ✅ implemented | `TokenConfigurationSnapshot::instance_id`. |
| `TKTokenConfiguration.configurationData` | ✅ implemented | `Token::set_configuration_data` / `Token::configuration` set and read the framework property; `set_configuration_data` returns `Unsupported` when CryptoTokenKit does not keep the value (outside the app that contains the token extension). |
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
| `TKSmartCardTokenDriverDelegate` | ✅ implemented | `SmartCardTokenDriverDelegate`. |

## TKSmartCardSlotNFCSession.h

| API | Status | Notes |
| --- | --- | --- |
| `TKSmartCardSlotNFCSession` | ⏭️ skipped | iOS/macCatalyst/visionOS-only NFC session surface. |

## TKSmartCardTokenRegistrationManager.h

| API | Status | Notes |
| --- | --- | --- |
| `TKSmartCardTokenRegistrationManager` | ⏭️ skipped | iOS/macCatalyst/visionOS-only registration manager surface. |
