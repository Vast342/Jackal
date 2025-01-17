use bullet::{
    inputs, loader, lr, optimiser, outputs, wdl, LocalSettings, Loss, TrainerBuilder,
    TrainingSchedule, TrainingSteps,
};

pub struct ValueTrainer;
impl ValueTrainer {
    pub fn execute() {
        let mut trainer = TrainerBuilder::default()
            .optimiser(optimiser::AdamW)
            .single_perspective()
            .loss_fn(Loss::SigmoidMSE)
            .input(inputs::ChessBucketsMirrored::default())
            .output_buckets(outputs::Single)
            .feature_transformer(64)
            .activate(bullet::Activation::SCReLU)
            .add_layer(1)
            .build();

        let schedule = TrainingSchedule {
            net_id: "value_009a".to_string(),
            eval_scale: 400.0,
            steps: TrainingSteps {
                batch_size: 16_384,
                batches_per_superbatch: 6104,
                start_superbatch: 1,
                end_superbatch: 50,
            },
            wdl_scheduler: wdl::ConstantWDL { value: 1.0 },
            lr_scheduler: lr::CosineDecayLR {
                initial_lr: 0.001,
                final_lr: 0.001 * 0.3 * 0.3 * 0.3,
                final_superbatch: 50,
            },
            save_rate: 10,
        };

        let settings = LocalSettings {
            threads: 16,
            test_set: None,
            output_directory: "checkpoints",
            batch_queue_size: 512,
        };

        let data_loader = loader::DirectSequentialDataLoader::new(&["./shuffled_bullet_data.bin"]);

        //trainer.load_from_checkpoint("checkpoints/value_008-80");
        trainer.run(&schedule, &settings, &data_loader);
    }
}
